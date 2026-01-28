//! Numelace desktop application using egui/eframe.
//!
//! This is the main entry point for the desktop Numelace application.

use eframe::{
    NativeOptions,
    egui::{self, Vec2},
};

use crate::app::NumelaceApp;

mod action;
mod action_handler;
mod app;
mod game_factory;
mod persistence;
mod state;
mod ui;
mod view_model_builder;

const APP_ID: &str = "io.github.gifnksm.numelace";

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(APP_ID)
            .with_resizable(true)
            .with_inner_size(Vec2::new(800.0, 600.0))
            .with_min_inner_size(Vec2::new(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Numelace",
        options,
        Box::new(|cc| Ok(Box::new(NumelaceApp::new(cc)))),
    )
}
