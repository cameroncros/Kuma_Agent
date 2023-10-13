extern crate core;

#[cfg(target_os = "windows")]
mod winservice;
mod mainloop;
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

#[cfg(target_os = "linux")]
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    mainloop(running);
}

#[cfg(target_os = "macos")]
use std::sync::Arc;
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicBool};

#[cfg(target_os = "macos")]
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    mainloop(running);
}