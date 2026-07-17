//! Spawns the Vite dev server in debug builds before the overlay webview loads.

use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const VITE_PORT: u16 = 5173;
const VITE_START_TIMEOUT: Duration = Duration::from_secs(30);

pub fn ensure_vite_dev_server() {
    if !cfg!(debug_assertions) {
        return;
    }

    if TcpStream::connect(("127.0.0.1", VITE_PORT)).is_ok() {
        tracing::info!("Vite dev server already running on port {VITE_PORT}");
        return;
    }

    let ui_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../ui");
    tracing::info!("Starting Vite dev server in {}", ui_dir.display());

    Command::new("npm")
        .args(["run", "dev"])
        .current_dir(&ui_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn Vite dev server");

    let deadline = Instant::now() + VITE_START_TIMEOUT;
    while Instant::now() < deadline {
        if TcpStream::connect(("127.0.0.1", VITE_PORT)).is_ok() {
            tracing::info!("Vite dev server ready on port {VITE_PORT}");
            return;
        }
        thread::sleep(Duration::from_millis(200));
    }

    panic!("Vite dev server did not start within {VITE_START_TIMEOUT:?}");
}

pub fn webview_dev_url() -> String {
    format!("http://127.0.0.1:{VITE_PORT}/")
}

pub fn webview_prod_url() -> String {
    "index.html".into()
}
