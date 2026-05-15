#![deny(unsafe_code)]

use crate::app::App;

mod app;
mod consts;
mod gadgets;
mod idgen;

pub fn run() {
    use eframe::{NativeOptions, egui::ViewportBuilder, run_native};

    let _ = run_native(
        env!("CARGO_PKG_NAME"),
        NativeOptions {
            viewport: ViewportBuilder::default().with_maximized(true),
            persist_window: false,
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(App::default()))),
    );
}
