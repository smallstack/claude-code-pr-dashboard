mod commands;
mod github;
mod pty;

use pty::create_session_map;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(create_session_map())
        .invoke_handler(tauri::generate_handler![
            commands::check_auth,
            commands::list_prs,
            commands::get_pr_detail,
            commands::create_session,
            commands::write_session,
            commands::resize_session,
            commands::close_session,
            commands::auto_fix_pr,
            commands::open_claude,
            commands::open_docker_shell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
