use eframe::egui::{Button, CollapsingHeader, RichText, ScrollArea, Ui};

use crate::{
    action::{Action, ActionRequestQueue},
    state::{AppearanceSettings, AssistSettings, HighlightSettings, Settings, Theme},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Solved,
}

#[derive(Debug, Clone)]
pub struct SidebarViewModel<'a> {
    status: GameStatus,
    settings: &'a Settings,
}

impl<'a> SidebarViewModel<'a> {
    pub fn new(status: GameStatus, settings: &'a Settings) -> Self {
        Self { status, settings }
    }
}

pub fn show(ui: &mut Ui, vm: &SidebarViewModel, action_queue: &mut ActionRequestQueue) {
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
                action_queue.request(Action::RequestNewGameConfirm);
            }
        });

        let mut changed = false;
        let mut settings = vm.settings.clone();
        let Settings { assist, appearance } = &mut settings;
        ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Settings");
            ui.indent("sidebar_settings", |ui| {
                let AssistSettings {
                    block_rule_violations,
                    highlight,
                } = assist;
                CollapsingHeader::new("Assist")
                    .default_open(true)
                    .show(ui, |ui| {
                        changed |= ui
                            .checkbox(block_rule_violations, "Block rule violations")
                            .changed();

                        ui.label("Highlight");
                        ui.indent("highlight", |ui| {
                            let HighlightSettings {
                                same_digit,
                                house_selected,
                                house_same_digit,
                                conflict,
                            } = highlight;
                            changed |= ui.checkbox(same_digit, "Same digit cells/notes").changed();
                            changed |= ui.checkbox(conflict, "Conflicting cells/notes").changed();
                            ui.label(RichText::new("Row/Col/Box Highlight"));
                            ui.indent("house_highlight", |ui| {
                                changed |= ui.checkbox(house_selected, "Selected cell").changed();
                                changed |=
                                    ui.checkbox(house_same_digit, "Same digit cells").changed();
                            });
                        });
                    });

                let AppearanceSettings { theme } = appearance;
                CollapsingHeader::new("Appearance")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Theme"));
                        ui.indent("theme", |ui| {
                            changed |= ui.radio_value(theme, Theme::Light, "Light").changed();
                            changed |= ui.radio_value(theme, Theme::Dark, "Dark").changed();
                        });
                    });
            });
        });
        if changed {
            action_queue.request(Action::UpdateSettings(settings));
        }
    });
}
