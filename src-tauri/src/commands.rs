use crate::github::{self, AuthStatus, PullRequest};
use crate::pty::{self, SessionMap};
use tauri::{AppHandle, State};

#[tauri::command]
pub fn check_auth() -> AuthStatus {
    AuthStatus {
        gh: github::check_gh_auth(),
        claude: github::check_claude_auth(),
    }
}

#[tauri::command]
pub fn list_prs(repo: String) -> Result<Vec<PullRequest>, String> {
    github::fetch_prs(&repo)
}

#[tauri::command]
pub fn get_pr_detail(repo: String, number: u32) -> Result<PullRequest, String> {
    github::fetch_pr_detail(&repo, number)
}

#[tauri::command]
pub fn create_session(
    app: AppHandle,
    sessions: State<'_, SessionMap>,
    id: String,
    cwd: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    pty::spawn_session(&app, &sessions, &id, &cwd, cols, rows)
}

#[tauri::command]
pub fn write_session(sessions: State<'_, SessionMap>, id: String, data: String) -> Result<(), String> {
    pty::write_to_session(&sessions, &id, &data)
}

#[tauri::command]
pub fn resize_session(
    sessions: State<'_, SessionMap>,
    id: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    pty::resize_session(&sessions, &id, cols, rows)
}

#[tauri::command]
pub fn close_session(sessions: State<'_, SessionMap>, id: String) -> Result<(), String> {
    pty::close_session(&sessions, &id)
}

#[tauri::command]
pub fn auto_fix_pr(
    sessions: State<'_, SessionMap>,
    id: String,
    repo_path: String,
    branch: String,
) -> Result<(), String> {
    // Change to repo directory
    pty::write_to_session(&sessions, &id, &format!("cd {}\n", repo_path))?;

    // Checkout the PR branch
    pty::write_to_session(&sessions, &id, &format!("git checkout {}\n", branch))?;

    // Pull latest
    pty::write_to_session(&sessions, &id, "git pull\n")?;

    // Start Claude Code and run /fix-pr
    pty::write_to_session(&sessions, &id, "claude\n")?;

    // Give Claude a moment to start, then send the command
    std::thread::sleep(std::time::Duration::from_secs(3));
    pty::write_to_session(&sessions, &id, "/fix-pr\n")?;

    Ok(())
}

#[tauri::command]
pub fn open_claude(
    sessions: State<'_, SessionMap>,
    id: String,
    repo_path: String,
) -> Result<(), String> {
    pty::write_to_session(&sessions, &id, &format!("cd {} && claude\n", repo_path))?;
    Ok(())
}
