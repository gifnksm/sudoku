//! Numelace desktop application UI.
//!
//! # Design Notes
//! - Desktop-focused MVP with a 9x9 grid and clear 3x3 boundaries.
//! - Keyboard-driven input (digits, arrows, delete/backspace) with mouse selection.
//! - Status display derived from `Game::is_solved()`.
//!
//! # Future Enhancements
//! - Candidate marks, undo/redo, hints, mistake detection.
//! - Save/load, timer/statistics, and web/WASM support.

use std::time::Duration;

use eframe::{
    App, CreationContext, Frame, Storage,
    egui::{CentralPanel, Context, Visuals},
};
use numelace_core::{Digit, Position};
use numelace_game::Game;
use numelace_generator::PuzzleGenerator;
use numelace_solver::TechniqueSolver;

use crate::{
    persistence::storage,
    state::{AppState, Theme, UiState},
    ui::{
        self, Action, MoveDirection, game_screen::GameScreenViewModel, grid::GridViewModel,
        keypad::KeypadViewModel, sidebar::SidebarViewModel,
    },
};

#[derive(Debug)]
pub struct NumelaceApp {
    app_state: AppState,
    ui_state: UiState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Solved,
}

impl NumelaceApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let app_state = cc.storage.and_then(storage::load_state).unwrap_or_else(|| {
            let theme = if cc.egui_ctx.style().visuals.dark_mode {
                Theme::Dark
            } else {
                Theme::Light
            };
            AppState::new(new_game(), theme)
        });
        let this = Self {
            app_state,
            ui_state: UiState::default(),
        };
        this.update_theme(&cc.egui_ctx);
        this
    }

    fn status(&self) -> GameStatus {
        if self.app_state.game.is_solved() {
            GameStatus::Solved
        } else {
            GameStatus::InProgress
        }
    }

    fn new_game(&mut self) {
        self.app_state.game = new_game();
        self.app_state.selected_cell = None;
    }

    fn set_digit(&mut self, digit: Digit) {
        if let Some(pos) = self.app_state.selected_cell {
            let _ = self.app_state.game.set_digit(pos, digit);
        }
    }

    fn remove_digit(&mut self) {
        if let Some(pos) = self.app_state.selected_cell {
            let _ = self.app_state.game.remove_digit(pos);
        }
    }

    fn apply_action(&mut self, action: Action, ctx: &Context) {
        const DEFAULT_POSITION: Position = Position::new(0, 0);
        match action {
            Action::SelectCell(pos) => {
                self.app_state.selected_cell = Some(pos);
            }
            Action::ClearSelection => {
                self.app_state.selected_cell = None;
            }
            Action::MoveSelection(move_direction) => {
                let pos = self.app_state.selected_cell.get_or_insert(DEFAULT_POSITION);
                let new_pos = match move_direction {
                    MoveDirection::Up => pos.up(),
                    MoveDirection::Down => pos.down(),
                    MoveDirection::Left => pos.left(),
                    MoveDirection::Right => pos.right(),
                };
                if let Some(new_pos) = new_pos {
                    *pos = new_pos;
                }
            }
            Action::SetDigit(digit) => {
                self.set_digit(digit);
            }
            Action::RemoveDigit => {
                self.remove_digit();
            }
            Action::RequestNewGameConfirm => {
                self.ui_state.show_new_game_confirm_dialogue = true;
            }
            Action::NewGame => {
                self.new_game();
            }
            Action::UpdateHighlightSettings(settings) => {
                self.app_state.settings.highlight = settings;
            }
            Action::UpdateThemeSettings(settings) => {
                self.app_state.settings.theme = settings;
                self.update_theme(ctx);
            }
        }
    }

    fn update_theme(&self, ctx: &Context) {
        match self.app_state.settings.theme.theme {
            Theme::Light => {
                ctx.set_visuals(Visuals::light());
            }
            Theme::Dark => {
                ctx.set_visuals(Visuals::dark());
            }
        }
    }
}

fn new_game() -> Game {
    let technique_solver = TechniqueSolver::with_all_techniques();
    let puzzle = PuzzleGenerator::new(&technique_solver).generate();
    Game::new(puzzle)
}

impl App for NumelaceApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        storage::save_state(storage, &self.app_state);
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(30)
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        let mut save_requested = false;
        if !self.ui_state.show_new_game_confirm_dialogue {
            ctx.input(|i| {
                for action in ui::input::handle_input(i) {
                    save_requested = true;
                    self.apply_action(action, ctx);
                }
            });
        }

        let game = &self.app_state.game;
        let selected_cell = self.app_state.selected_cell;
        let settings = &self.app_state.settings;
        let can_set_digit = selected_cell.is_some_and(|pos| game.can_set_digit(pos));
        let has_removable_digit = selected_cell.is_some_and(|pos| game.has_removable_digit(pos));
        let selected_digit = selected_cell.and_then(|pos| game.cell(pos).as_digit());
        let grid_vm = GridViewModel::new(game, selected_cell, selected_digit, &settings.highlight);
        let keypad_vm = KeypadViewModel::new(
            can_set_digit,
            has_removable_digit,
            game.decided_digit_count(),
        );
        let sidebar_vm = SidebarViewModel::new(self.status(), &settings.highlight, &settings.theme);
        let game_screen_vm = GameScreenViewModel::new(grid_vm, keypad_vm, sidebar_vm);

        let mut actions = vec![];
        CentralPanel::default().show(ctx, |ui| {
            actions = ui::game_screen::show(ui, &game_screen_vm);
            if self.ui_state.show_new_game_confirm_dialogue {
                actions.extend(ui::dialogs::show_new_game_confirm(
                    ui,
                    &mut self.ui_state.show_new_game_confirm_dialogue,
                ));
            }
        });

        for action in actions {
            save_requested = true;
            self.apply_action(action, ctx);
        }

        if save_requested && let Some(storage) = frame.storage_mut() {
            self.save(storage);
        }
    }
}
