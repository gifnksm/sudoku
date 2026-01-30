use eframe::egui::{Align, Layout, RichText, Ui};

use crate::ui::icon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Solved,
}

#[derive(Debug, Clone)]
pub struct StatusLineViewModel {
    status: GameStatus,
}

impl StatusLineViewModel {
    pub fn new(status: GameStatus) -> Self {
        Self { status }
    }
}

pub fn show(ui: &mut Ui, vm: &StatusLineViewModel) {
    let h = ui.available_height();
    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
        ui.add_space(h * 0.25);
        ui.horizontal(|ui| {
            let (status_text, status_color) = match vm.status {
                GameStatus::InProgress => (
                    format!("{} Game in progress...", icon::HOURGLASS),
                    ui.visuals().text_color(),
                ),
                GameStatus::Solved => (
                    format!("{} Solved! Congratulations!", icon::TROPHY),
                    ui.visuals().warn_fg_color,
                ),
            };
            ui.label(RichText::new(status_text).color(status_color).size(h * 0.5));
        });
    });
}
