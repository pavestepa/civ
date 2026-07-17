pub const OP: &str = "InputFrame";

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputFrame {
    pub keys: Vec<String>,
    pub scroll_delta: f32,
    #[serde(default)]
    pub mouse_click: bool,
    #[serde(default)]
    pub mouse_x: f32,
    #[serde(default)]
    pub mouse_y: f32,
}
