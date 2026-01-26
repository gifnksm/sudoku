use eframe::egui::{Button, CollapsingHeader, RichText, ScrollArea, Ui};

use crate::{
    app::GameStatus,
    state::{HighlightSettings, Theme, ThemeSettings},
    ui::Action,
};

#[derive(Debug, Clone)]
pub struct SidebarViewModel<'a> {
    status: GameStatus,
    highlight_settings: &'a HighlightSettings,
    theme_settings: &'a ThemeSettings,
}

impl<'a> SidebarViewModel<'a> {
    pub fn new(
        status: GameStatus,
        highlight_settings: &'a HighlightSettings,
        theme_settings: &'a ThemeSettings,
    ) -> Self {
        Self {
            status,
            highlight_settings,
            theme_settings,
        }
    }
}

pub fn show(ui: &mut Ui, vm: &SidebarViewModel) -> Vec<Action> {
    let mut actions = vec![];
    ui.vertical(|ui| {
        ui.group(|ui| {
            let status_text = match vm.status {
                GameStatus::InProgress => "Game in progress",
                GameStatus::Solved => "Congratulations! You solved the puzzle!",
            };
            let status_label = match vm.status {
                GameStatus::InProgress => RichText::new(status_text),
                GameStatus::Solved => RichText::new(status_text).color(ui.visuals().warn_fg_color),
            };
            ui.label(status_label.size(20.0));
            ui.add_space(8.0);
            let button = ui.add_sized(
                [ui.available_width(), 36.0],
                Button::new(RichText::new("New Game").size(20.0)),
            );
            if button.clicked() {
                actions.push(Action::RequestNewGameConfirm);
            }
        });

        ScrollArea::vertical().show(ui, |ui| {
            let mut hls = vm.highlight_settings.clone();
            let mut hls_changed = false;
            CollapsingHeader::new("Highlight settings")
                .default_open(true)
                .show(ui, |ui| {
                    hls_changed |= ui
                        .checkbox(&mut hls.same_digit, "Same digit cells")
                        .changed();
                    ui.label(RichText::new("Row/Col/Box Highlight"));
                    ui.indent("rcb_highlight", |ui| {
                        hls_changed |= ui
                            .checkbox(&mut hls.rcb_selected, "Selected cell")
                            .changed();
                        hls_changed |= ui
                            .checkbox(&mut hls.rcb_same_digit, "Same digit cells")
                            .changed();
                    });
                    if hls_changed {
                        actions.push(Action::UpdateHighlightSettings(hls));
                    }
                });

            let mut ts = vm.theme_settings.clone();
            let mut ts_changed = false;
            CollapsingHeader::new("Appearance settings")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(RichText::new("Theme"));
                    ui.indent("theme", |ui| {
                        ts_changed |= ui
                            .radio_value(&mut ts.theme, Theme::Light, "Light")
                            .changed();
                        ts_changed |= ui.radio_value(&mut ts.theme, Theme::Dark, "Dark").changed();
                    });
                });
            if ts_changed {
                actions.push(Action::UpdateThemeSettings(ts));
            }
        });
    });
    actions
}
