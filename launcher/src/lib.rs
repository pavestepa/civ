mod commands;
mod ipc;
mod state;

use tauri::Manager;

pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state::LauncherState::default())
        .invoke_handler(tauri::generate_handler![
            commands::dispatch_command,
            commands::poll_events,
            commands::launch_engine,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
