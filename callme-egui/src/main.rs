#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use callme_egui::app::App;
use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    // RUST_LOG=debug cargo run --bin callme-egui
    // RUST_LOG=callme=trace,callme_egui=debug cargo run --bin callme-egui
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(true)
        .with_level(true)
        .init();

    let mut options = NativeOptions::default();
    options.viewport = options
        .viewport
        .with_title("Callme")
        .with_resizable(true)
        .with_inner_size([500., 600.]);
    App::run(options)
}
