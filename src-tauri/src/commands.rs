use crate::github::{self, AuthStatus, PullRequest};
use crate::pty::{self, DockerConfig, SessionMap};
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
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    pty::spawn_shell_session(&app, &sessions, &id, cols, rows)
}

#[tauri::command]
pub fn write_session(
    sessions: State<'_, SessionMap>,
    id: String,
    data: String,
) -> Result<(), String> {
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
pub fn open_claude(
    app: AppHandle,
    sessions: State<'_, SessionMap>,
    id: String,
    repo: String,
    github_token: String,
    claude_credentials_path: String,
    claude_model: String,
    git_user_name: String,
    git_user_email: String,
    docker_template: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let config = DockerConfig {
        repo,
        branch: None,
        github_token,
        claude_credentials_path,
        claude_model,
        git_user_name,
        git_user_email,
        docker_template,
    };
    pty::spawn_docker_session(&app, &sessions, &id, &config, cols, rows)
}

#[tauri::command]
pub fn auto_fix_pr(
    app: AppHandle,
    sessions: State<'_, SessionMap>,
    id: String,
    repo: String,
    branch: String,
    github_token: String,
    claude_credentials_path: String,
    claude_model: String,
    git_user_name: String,
    git_user_email: String,
    docker_template: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let config = DockerConfig {
        repo,
        branch: Some(branch),
        github_token,
        claude_credentials_path,
        claude_model,
        git_user_name,
        git_user_email,
        docker_template,
    };
    pty::spawn_docker_session(&app, &sessions, &id, &config, cols, rows)
}
