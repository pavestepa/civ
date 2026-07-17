use serde::Deserialize;

use crate::ipc::parse_command;
use crate::state::LauncherState;

#[derive(Debug, Deserialize)]
#[serde(tag = "action", content = "data")]
pub enum UiAction {
    EndTurn,
    AddGold { player: u32, amount: i32 },
    SpawnUnit { owner: u32, q: i32, r: i32, kind: String },
    CreateCity { owner: u32, q: i32, r: i32, name: String },
    RequestSnapshot,
}

#[tauri::command]
pub fn dispatch_command(
    state: tauri::State<'_, LauncherState>,
    action: UiAction,
) -> Result<u64, String> {
    let ui_command = parse_command(action);
    state.send(civ_ui_bridge::IpcMessage::Command(ui_command))
}

#[tauri::command]
pub fn poll_events(state: tauri::State<'_, LauncherState>) -> Result<Vec<String>, String> {
    state.poll()
}

#[tauri::command]
pub fn launch_engine() -> Result<String, String> {
    Ok("engine launch requested — run `cargo run -p civ-engine` alongside launcher".into())
}
