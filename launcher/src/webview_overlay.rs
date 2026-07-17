//! Embeds a transparent wry WebView as a child of the Bevy primary window.
//!
//! wry attaches to the existing winit window — no nested Tao/Tauri event loop.

use std::sync::Arc;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy::winit::WinitWindows;
use civ_ui_bridge::UiChannelEndpoint;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::http::Request;
use wry::{Rect, WebViewBuilder};

use crate::commands::UiAction;
use crate::dev_server::{ensure_vite_dev_server, webview_dev_url, webview_prod_url};
use crate::ipc::parse_command;
use crate::state::LauncherIpc;

const INIT_SCRIPT: &str = r#"
window.__civReceiveEvents = function(events) {
  window.dispatchEvent(new CustomEvent('civ:events', { detail: events }));
};
"#;

/// Bevy resource wrapping shared launcher IPC state.
#[derive(Resource, Clone)]
struct LauncherIpcResource(LauncherIpc);

/// Holds the launcher-side IPC endpoint until the overlay webview initializes.
#[derive(Resource, Clone)]
pub struct LauncherChannel(pub UiChannelEndpoint);

pub struct OverlayWebView(wry::WebView);

#[derive(Resource, Default)]
struct WebViewOverlayState {
    attached: bool,
    warmup_frames: u32,
}

pub struct WebViewOverlayPlugin;

impl Plugin for WebViewOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WebViewOverlayState>().add_systems(
            Update,
            (
                try_attach_webview_overlay,
                push_events_to_webview,
                sync_webview_bounds,
            ),
        );
    }
}

fn try_attach_webview_overlay(world: &mut World) {
    {
        let mut state = world.resource_mut::<WebViewOverlayState>();
        if state.attached {
            return;
        }

        // Wait until winit finishes showing the window (it starts hidden for a11y setup).
        state.warmup_frames += 1;
        if state.warmup_frames < 3 {
            return;
        }
    }

    let Some(launcher_channel) = world.get_resource::<LauncherChannel>() else {
        tracing::warn!("LauncherChannel not set — skipping UI overlay");
        return;
    };

    let launcher_endpoint = launcher_channel.0.clone();

    let entity = {
        let mut query = world.query_filtered::<Entity, With<PrimaryWindow>>();
        match query.iter(world).next() {
            Some(entity) => entity,
            None => return,
        }
    };

    let (width, height) = {
        let winit_windows = world.non_send_resource::<WinitWindows>();
        let Some(winit_window) = winit_windows.get_window(entity) else {
            return;
        };

        let size = winit_window.inner_size();
        (size.width as f64, size.height as f64)
    };

    if cfg!(debug_assertions) {
        ensure_vite_dev_server();
    }

    let url = if cfg!(debug_assertions) {
        webview_dev_url()
    } else {
        webview_prod_url()
    };

    tracing::info!("Attaching UI overlay ({width}x{height})");

    let launcher_ipc = LauncherIpc::new(launcher_endpoint);
    let ipc_state = Arc::clone(&launcher_ipc.0);

    let webview = {
        let winit_windows = world.non_send_resource::<WinitWindows>();
        let winit_window = winit_windows
            .get_window(entity)
            .expect("primary winit window");

        let winit_window: &winit::window::Window = winit_window;

        WebViewBuilder::new()
            .with_url(&url)
            .with_transparent(true)
            .with_bounds(Rect {
                position: LogicalPosition::new(0.0, 0.0).into(),
                size: LogicalSize::new(width, height).into(),
            })
            .with_initialization_script(INIT_SCRIPT)
            .with_ipc_handler(move |request: Request<String>| {
                let body = request.body();
                if let Ok(action) = serde_json::from_str::<UiAction>(body) {
                    let command = parse_command(action);
                    let _ = ipc_state.send(civ_ui_bridge::IpcMessage::Command(command));
                } else {
                    tracing::warn!("unrecognized IPC message: {body}");
                }
            })
            .build_as_child(winit_window)
            .expect("failed to create UI overlay webview")
    };

    #[cfg(debug_assertions)]
    webview.open_devtools();

    world.insert_resource(LauncherIpcResource(launcher_ipc));
    world.insert_non_send_resource(OverlayWebView(webview));
    world.resource_mut::<WebViewOverlayState>().attached = true;

    tracing::info!("UI overlay attached to Bevy window");
}

fn push_events_to_webview(
    webview: Option<NonSend<OverlayWebView>>,
    state: Option<Res<LauncherIpcResource>>,
) {
    let Some(webview) = webview else {
        return;
    };
    let Some(state) = state else {
        return;
    };

    let Ok(events) = state.0.poll() else {
        return;
    };
    if events.is_empty() {
        return;
    }

    let Ok(json) = serde_json::to_string(&events) else {
        return;
    };
    let js = format!("window.__civReceiveEvents({json})");
    if let Err(err) = webview.0.evaluate_script(&js) {
        tracing::warn!("failed to push events to webview: {err}");
    }
}

fn sync_webview_bounds(
    webview: Option<NonSend<OverlayWebView>>,
    mut resize_events: EventReader<WindowResized>,
    primary: Query<(Entity, &Window), With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    let Some(webview) = webview else {
        return;
    };

    for event in resize_events.read() {
        let Ok((entity, _)) = primary.get(event.window) else {
            continue;
        };
        let Some(winit_window) = winit_windows.get_window(entity) else {
            continue;
        };

        let size = winit_window.inner_size();
        let bounds = Rect {
            position: LogicalPosition::new(0.0, 0.0).into(),
            size: LogicalSize::new(size.width as f64, size.height as f64).into(),
        };
        if let Err(err) = webview.0.set_bounds(bounds) {
            tracing::warn!("failed to resize overlay webview: {err}");
        }
    }
}
