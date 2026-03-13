use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter};

pub struct PtySession {
    writer: Box<dyn Write + Send>,
    _master: Box<dyn MasterPty + Send>,
}

pub type SessionMap = Arc<Mutex<HashMap<String, PtySession>>>;

pub fn create_session_map() -> SessionMap {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Resolve the absolute path to the docker/ directory within the app.
/// In dev mode, this is relative to the project root.
/// In production, it's bundled as a Tauri resource.
fn docker_dir() -> String {
    // In dev, CARGO_MANIFEST_DIR points to src-tauri/
    // The docker dir is at the project root: ../docker/
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
        .parent()
        .unwrap()
        .join("docker");
    path.to_string_lossy().to_string()
}

/// Check if docker is accessible without sudo
fn needs_sudo_for_docker() -> bool {
    std::process::Command::new("docker")
        .arg("info")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| !s.success())
        .unwrap_or(true)
}

pub struct DockerConfig {
    pub repo: String,
    pub branch: Option<String>,
    pub github_token: String,
    pub claude_credentials_path: String,
    pub claude_model: String,
    pub git_user_name: String,
    pub git_user_email: String,
}

/// Expand ~ at the start of a path to the user's home directory.
fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return format!("{}/{}", home.to_string_lossy(), rest);
        }
    } else if path == "~" {
        if let Some(home) = std::env::var_os("HOME") {
            return home.to_string_lossy().to_string();
        }
    }
    path.to_string()
}

pub fn spawn_docker_session(
    app: &AppHandle,
    sessions: &SessionMap,
    id: &str,
    config: &DockerConfig,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    if config.github_token.is_empty() {
        return Err("GitHub token is not configured. Open Settings to set it.".to_string());
    }
    if config.claude_credentials_path.is_empty() {
        return Err("Claude credentials path is not configured. Open Settings to set it.".to_string());
    }

    let pty_system = native_pty_system();

    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {}", e))?;

    let docker_path = docker_dir();
    let credentials_path = expand_tilde(&config.claude_credentials_path);
    let use_sudo = needs_sudo_for_docker();

    let mut cmd = if use_sudo {
        let mut c = CommandBuilder::new("sudo");
        c.args(["-E", "docker"]);
        c
    } else {
        CommandBuilder::new("docker")
    };
    cmd.args([
        "compose",
        "-f",
        &format!("{}/docker-compose.yml", docker_path),
        "run",
        "--rm",
        "--service-ports",
        "-v", &format!("{}:/home/claude/.claude-host-credentials.json:ro", credentials_path),
        "-e", &format!("REPO={}", config.repo),
        "-e", &format!("GITHUB_TOKEN={}", config.github_token),
        "-e", &format!("GIT_USER_NAME={}", config.git_user_name),
        "-e", &format!("GIT_USER_EMAIL={}", config.git_user_email),
        "-e", &format!("CLAUDE_MODEL={}", config.claude_model),
    ]);

    if let Some(ref b) = config.branch {
        cmd.args(["-e", &format!("BRANCH={}", b)]);
    }

    cmd.arg("claude");

    // Set env vars on the process for docker-compose.yml interpolation
    cmd.env("GITHUB_TOKEN", &config.github_token);
    cmd.env("CLAUDE_CREDENTIALS", &config.claude_credentials_path);

    let _child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Failed to spawn docker session: {}", e))?;

    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone reader: {}", e))?;

    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("Failed to take writer: {}", e))?;

    {
        let mut map = sessions.lock().unwrap();
        map.insert(
            id.to_string(),
            PtySession {
                writer,
                _master: pair.master,
            },
        );
    }

    let event_name = format!("pty-output-{}", id);
    let app_handle = app.clone();

    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app_handle.emit(&event_name, data);
                }
                Err(_) => break,
            }
        }
    });

    Ok(())
}

pub fn spawn_shell_session(
    app: &AppHandle,
    sessions: &SessionMap,
    id: &str,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let pty_system = native_pty_system();

    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {}", e))?;

    let cmd = CommandBuilder::new_default_prog();

    let _child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Failed to spawn shell: {}", e))?;

    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone reader: {}", e))?;

    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("Failed to take writer: {}", e))?;

    {
        let mut map = sessions.lock().unwrap();
        map.insert(
            id.to_string(),
            PtySession {
                writer,
                _master: pair.master,
            },
        );
    }

    let event_name = format!("pty-output-{}", id);
    let app_handle = app.clone();

    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app_handle.emit(&event_name, data);
                }
                Err(_) => break,
            }
        }
    });

    Ok(())
}

pub fn write_to_session(sessions: &SessionMap, id: &str, data: &str) -> Result<(), String> {
    let mut map = sessions.lock().unwrap();
    let session = map
        .get_mut(id)
        .ok_or_else(|| format!("Session {} not found", id))?;

    session
        .writer
        .write_all(data.as_bytes())
        .map_err(|e| format!("Failed to write to PTY: {}", e))?;

    session
        .writer
        .flush()
        .map_err(|e| format!("Failed to flush PTY: {}", e))?;

    Ok(())
}

pub fn close_session(sessions: &SessionMap, id: &str) -> Result<(), String> {
    let mut map = sessions.lock().unwrap();
    map.remove(id)
        .ok_or_else(|| format!("Session {} not found", id))?;
    Ok(())
}

pub fn resize_session(
    sessions: &SessionMap,
    id: &str,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let map = sessions.lock().unwrap();
    let session = map
        .get(id)
        .ok_or_else(|| format!("Session {} not found", id))?;

    session
        ._master
        .resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to resize PTY: {}", e))?;

    Ok(())
}
