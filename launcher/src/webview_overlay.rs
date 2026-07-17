//! Embeds a transparent wry WebView as a child of the Bevy primary window.
//!
//! wry attaches to the existing winit window — no nested Tao/Tauri event loop.

use std::sync::Arc;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy::winit::WinitWindows;
use civ_channel::WireMessage;
use civ_ui_bridge::UiChannelEndpoint;
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::http::Request;
use wry::{Rect, WebViewBuilder};

use crate::dev_server::{ensure_vite_dev_server, webview_dev_url, webview_prod_url};
use crate::state::LauncherIpc;

const INIT_SCRIPT: &str = r#"
(function () {
  const style = document.createElement("style");
  style.textContent = "html, body, #root { background: transparent !important; }";
  document.documentElement.appendChild(style);
  document.body.tabIndex = -1;
})();
"#;

#[derive(Resource, Clone)]
struct LauncherIpcResource(LauncherIpc);

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
                push_messages_to_webview,
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

        state.warmup_frames += 1;
        if state.warmup_frames < 3 {
            return;
        }
    }

    let launcher_endpoint = {
        let Some(launcher_channel) = world.get_resource::<LauncherChannel>() else {
            tracing::warn!("LauncherChannel not set — skipping UI overlay");
            return;
        };
        launcher_channel.0.clone()
    };

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
            .with_initialization_script(INIT_SCRIPT)
            .with_bounds(Rect {
                position: LogicalPosition::new(0.0, 0.0).into(),
                size: LogicalSize::new(width, height).into(),
            })
            .with_ipc_handler(move |request: Request<String>| {
                let body = request.body();
                match WireMessage::from_json(body) {
                    Ok(WireMessage::Req { id, op, body }) => {
                        if let Err(error) = ipc_state.forward_request(id, op, body) {
                            tracing::warn!("failed to forward front-api request: {error}");
                        }
                    }
                    Ok(WireMessage::Evt { op, body }) => {
                        if let Err(error) = ipc_state.forward_event(op, body) {
                            tracing::warn!("failed to forward ui event: {error}");
                        }
                    }
                    Ok(other) => {
                        tracing::warn!("unexpected wire message from webview: {other:?}");
                    }
                    Err(error) => {
                        tracing::warn!("unrecognized IPC message ({error}): {body}");
                    }
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

fn push_messages_to_webview(
    webview: Option<NonSend<OverlayWebView>>,
    state: Option<Res<LauncherIpcResource>>,
) {
    let Some(webview) = webview else {
        return;
    };
    let Some(state) = state else {
        return;
    };

    let Ok(messages) = state.0.poll_outbox() else {
        return;
    };

    for message in messages {
        let Ok(json) = message.to_json() else {
            continue;
        };
        let js = match message {
            WireMessage::Res { .. } => format!("window.__civReceiveResponse({json})"),
            WireMessage::Evt { .. } => format!("window.__civReceiveEvent({json})"),
            WireMessage::Req { .. } => continue,
        };
        if let Err(err) = webview.0.evaluate_script(&js) {
            tracing::warn!("failed to push message to webview: {err}");
        }
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
