extern crate core;

#[cfg(target_os = "windows")]
mod winservice;
mod mainloop;
mod api;

use crate::mainloop::mainloop;

#[cfg(target_os = "windows")]
use crate::winservice::kuma_uptime_service;

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    kuma_uptime_service::run()
}


#[cfg(target_os = "linux")]
use std::sync::Arc;
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_os = "linux")]
use ctrlc;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use tokio::select;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use tokio_util::sync::CancellationToken;

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    let running = CancellationToken::new();
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.cancel()
    }).expect("Error setting Ctrl-C handler");

    select! {
        () = mainloop() => {},
        _ = api::api() => {},
        () = running.cancelled() => {}
    }
}

#[cfg(target_os = "macos")]
#[tokio::main]
async fn main() {
    let running = CancellationToken::new();
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.cancel()
    }).expect("Error setting Ctrl-C handler");

    select! {
        () = mainloop() => {},
        _ = api::api() => {},
        () = running.cancelled() => {}
    }
}