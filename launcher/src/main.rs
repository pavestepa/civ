use civ_ui_bridge::UiChannel;

fn main() {
    let (engine_channel, launcher_endpoint) = UiChannel::paired();

    civ_engine::build_app(engine_channel)
        .insert_resource(civ_launcher_lib::LauncherChannel(launcher_endpoint))
        .add_plugins(civ_launcher_lib::WebViewOverlayPlugin)
        .run();
}
