use std::sync::Arc;

use eframe::egui::{self, Align2, Button, FontId, Grid, Layout, RichText, Ui, Vec2};
use numelace_core::{Digit, containers::Array9, index::DigitSemantics};

use crate::ui::Action;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct KeypadCapabilities: u8 {
        const CAN_TOGGLE_DIGIT = 0b0000_0001;
        const CAN_TOGGLE_NOTE = 0b0000_0010;
        const HAS_REMOVABLE_DIGIT = 0b0000_0100;
    }
}

#[derive(Debug, Clone)]
pub struct KeypadViewModel {
    capabilities: KeypadCapabilities,
    decided_digit_count: Array9<usize, DigitSemantics>,
    notes_mode: bool,
}

impl KeypadViewModel {
    pub fn new(
        capabilities: KeypadCapabilities,
        decided_digit_count: Array9<usize, DigitSemantics>,
        notes_mode: bool,
    ) -> Self {
        Self {
            capabilities,
            decided_digit_count,
            notes_mode,
        }
    }

    fn can_toggle_digit(&self) -> bool {
        self.capabilities
            .contains(KeypadCapabilities::CAN_TOGGLE_DIGIT)
    }

    fn can_toggle_note(&self) -> bool {
        self.capabilities
            .contains(KeypadCapabilities::CAN_TOGGLE_NOTE)
    }

    fn has_removable_digit(&self) -> bool {
        self.capabilities
            .contains(KeypadCapabilities::HAS_REMOVABLE_DIGIT)
    }
}

enum ButtonType {
    Digit(Digit),
    RemoveDigit,
}

const BUTTON_LAYOUT: [[ButtonType; 5]; 2] = {
    const fn d(d: Digit) -> ButtonType {
        ButtonType::Digit(d)
    }
    const fn r() -> ButtonType {
        ButtonType::RemoveDigit
    }

    #[allow(clippy::enum_glob_use)]
    use Digit::*;
    [
        [d(D1), d(D2), d(D3), d(D4), d(D5)],
        [d(D6), d(D7), d(D8), d(D9), r()],
    ]
};

pub fn show(ui: &mut Ui, vm: &KeypadViewModel) -> Vec<Action> {
    let mut actions = vec![];

    let style = Arc::clone(ui.style());
    let visuals = &style.visuals;
    let digit_count_color = visuals.text_color();

    let x_padding = 5.0;
    let y_padding = 5.0;
    let avail = ui.available_size();
    let button_size = f32::min(
        (avail.x - 4.0 * x_padding) / 5.0,
        (avail.y - y_padding) / 2.0,
    );
    let buttons_width = button_size * 5.0 + x_padding * 4.0;
    let left_pad = (avail.x - buttons_width) / 2.0;
    let swap_input_mode = ui.input(|i| i.modifiers.command);
    let effective_notes_mode = vm.notes_mode ^ swap_input_mode;

    ui.horizontal(|ui| {
        ui.add_space(left_pad);
        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            Grid::new(ui.id().with("keypad_grid"))
                .spacing((x_padding, y_padding))
                .max_col_width(button_size)
                .show(ui, |ui| {
                    for row in BUTTON_LAYOUT {
                        for button_type in row {
                            match button_type {
                                ButtonType::Digit(digit) => {
                                    let (enabled, tooltip) = if effective_notes_mode {
                                        (vm.can_toggle_note(), "Toggle note")
                                    } else {
                                        (vm.can_toggle_digit(), "Toggle digit")
                                    };
                                    let text =
                                        RichText::new(digit.as_str()).size(button_size * 0.8);
                                    let button =
                                        Button::new(text).min_size(Vec2::splat(button_size));
                                    let button =
                                        ui.add_enabled(enabled, button).on_hover_text(tooltip);
                                    if button.clicked() {
                                        actions.push(Action::RequestDigit {
                                            digit,
                                            swap: swap_input_mode,
                                        });
                                    }
                                    ui.painter().text(
                                        button.rect.right_top() + egui::vec2(-4.0, 2.0),
                                        Align2::RIGHT_TOP,
                                        vm.decided_digit_count[digit].to_string(),
                                        FontId::proportional(button_size * 0.25),
                                        digit_count_color,
                                    );
                                }
                                ButtonType::RemoveDigit => {
                                    let text = RichText::new("X").size(button_size * 0.8);
                                    let button =
                                        Button::new(text).min_size(Vec2::splat(button_size));
                                    let button = ui
                                        .add_enabled(vm.has_removable_digit(), button)
                                        .on_hover_text("Remove digit/notes");
                                    if button.clicked() {
                                        actions.push(Action::ClearCell);
                                    }
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let swap_marker = if swap_input_mode { "^" } else { "" };
                ui.label(RichText::new("Effective Mode:").size(button_size * 0.2));
                if effective_notes_mode {
                    ui.label(
                        RichText::new(format!("{swap_marker}Notes"))
                            .size(button_size * 0.2)
                            .background_color(visuals.selection.bg_fill),
                    );
                } else {
                    ui.label(RichText::new(format!("{swap_marker}Fill")).size(button_size * 0.2));
                }
            });
            let mut notes_mode = vm.notes_mode;
            if ui
                .checkbox(
                    &mut notes_mode,
                    RichText::new("Fill/Notes").size(button_size * 0.2),
                )
                .changed()
            {
                actions.push(Action::ToggleInputMode);
            }
        });
    });
    actions
}
