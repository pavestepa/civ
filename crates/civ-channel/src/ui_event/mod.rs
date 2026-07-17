use std::collections::HashSet;

use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

#[path = "../../../../channel/ui-event/input_frame/mod.rs"]
pub mod input_frame;

pub use input_frame::{InputFrame, OP};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UiInputSet;

#[derive(Resource, Default)]
pub struct WebviewInputState {
    keys_pressed: HashSet<KeyCode>,
    scroll_delta: f32,
    mouse_click: bool,
    mouse_x: f32,
    mouse_y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseClick {
    pub x: f32,
    pub y: f32,
}

impl WebviewInputState {
    pub fn apply_frame(&mut self, frame: &InputFrame) {
        self.keys_pressed.clear();
        for key in &frame.keys {
            if let Some(code) = parse_key_code(key) {
                self.keys_pressed.insert(code);
            }
        }
        self.scroll_delta = frame.scroll_delta;
        self.mouse_click = frame.mouse_click;
        self.mouse_x = frame.mouse_x;
        self.mouse_y = frame.mouse_y;
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn take_scroll_delta(&mut self) -> f32 {
        std::mem::take(&mut self.scroll_delta)
    }

    pub fn take_click(&mut self) -> Option<MouseClick> {
        if !self.mouse_click {
            return None;
        }
        self.mouse_click = false;
        Some(MouseClick {
            x: self.mouse_x,
            y: self.mouse_y,
        })
    }
}

fn parse_key_code(code: &str) -> Option<KeyCode> {
    match code {
        "KeyW" => Some(KeyCode::KeyW),
        "KeyA" => Some(KeyCode::KeyA),
        "KeyS" => Some(KeyCode::KeyS),
        "KeyD" => Some(KeyCode::KeyD),
        "KeyQ" => Some(KeyCode::KeyQ),
        "KeyE" => Some(KeyCode::KeyE),
        "ArrowUp" => Some(KeyCode::ArrowUp),
        "ArrowDown" => Some(KeyCode::ArrowDown),
        "ArrowLeft" => Some(KeyCode::ArrowLeft),
        "ArrowRight" => Some(KeyCode::ArrowRight),
        _ => None,
    }
}
