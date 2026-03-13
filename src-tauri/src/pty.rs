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

pub fn spawn_session(
    app: &AppHandle,
    sessions: &SessionMap,
    id: &str,
    cwd: &str,
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

    let mut cmd = CommandBuilder::new_default_prog();
    cmd.cwd(cwd);

    let _child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Failed to spawn shell: {}", e))?;

    // Drop slave — we only need the master side
    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone reader: {}", e))?;

    let writer = pair
        .master
        .take_writer()
        .map_err(|e| format!("Failed to take writer: {}", e))?;

    // Store session
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

    // Spawn reader thread that emits output to frontend
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
