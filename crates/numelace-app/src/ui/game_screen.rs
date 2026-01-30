use eframe::egui::{self, Ui};
use egui_extras::{Size, StripBuilder};

use super::{grid, keypad};
use crate::{
    action::ActionRequestQueue,
    ui::{
        grid::GridViewModel,
        keypad::KeypadViewModel,
        status_line::{self, StatusLineViewModel},
    },
};

#[derive(Debug, Clone)]
pub struct GameScreenViewModel {
    pub status_line: StatusLineViewModel,
    pub grid: GridViewModel,
    pub keypad: KeypadViewModel,
}

impl GameScreenViewModel {
    pub fn new(
        status_line: StatusLineViewModel,
        grid: GridViewModel,
        keypad: KeypadViewModel,
    ) -> Self {
        Self {
            status_line,
            grid,
            keypad,
        }
    }
}

pub fn show(ui: &mut Ui, vm: &GameScreenViewModel, action_queue: &mut ActionRequestQueue) {
    let status_rows = 0.5;
    let grid_rows = 9.0;
    let keypad_rows = 2.0;
    let total_rows = status_rows + grid_rows + keypad_rows;

    let grid_ratio = egui::vec2(1.0, grid_rows / total_rows);
    let spacing = ui.spacing().item_spacing;
    let spaces = spacing * egui::vec2(2.0, 3.0);
    let grid_size = ((ui.available_size() - spaces) * grid_ratio).min_elem();
    let status_size = grid_size / grid_rows * status_rows;
    let keypad_size = grid_size / grid_rows * keypad_rows;

    StripBuilder::new(ui)
        .size(Size::remainder())
        .size(Size::exact(grid_size))
        .size(Size::remainder())
        .horizontal(|mut strip| {
            strip.empty();
            strip.cell(|ui| {
                StripBuilder::new(ui)
                    .size(Size::remainder())
                    .size(Size::exact(status_size))
                    .size(Size::exact(grid_size))
                    .size(Size::exact(spacing.y))
                    .size(Size::exact(keypad_size))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.empty();
                        strip.cell(|ui| {
                            status_line::show(ui, &vm.status_line);
                        });
                        strip.cell(|ui| {
                            grid::show(ui, &vm.grid, action_queue);
                        });
                        strip.cell(|_ui| {}); // Spacer
                        strip.cell(|ui| {
                            keypad::show(ui, &vm.keypad, action_queue);
                        });
                        strip.empty();
                    });
            });
            strip.empty();
        });
}
