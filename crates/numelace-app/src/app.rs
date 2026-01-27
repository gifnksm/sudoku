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
use numelace_core::{
    Digit, Position,
    containers::{Array9, Array81},
    index::PositionSemantics,
};
use numelace_game::{CellState, Game, GameError, RuleCheckPolicy};
use numelace_generator::PuzzleGenerator;
use numelace_solver::TechniqueSolver;

use crate::{
    persistence::storage,
    state::{AppState, GhostType, InputMode, Theme, UiState},
    ui::{
        self, Action, MoveDirection,
        game_screen::GameScreenViewModel,
        grid::{GridCell, GridViewModel, GridVisualState, NoteVisualState},
        keypad::{DigitKeyState, KeypadViewModel},
        sidebar::SidebarViewModel,
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
        let app_state = cc
            .storage
            .and_then(storage::load_state)
            .unwrap_or_else(|| AppState::new(new_game()));
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

    fn request_digit(&mut self, digit: Digit, swap: bool) {
        if let Some(pos) = self.app_state.selected_cell {
            let policy = if self.app_state.settings.assist.block_rule_violations {
                RuleCheckPolicy::Strict
            } else {
                RuleCheckPolicy::Permissive
            };
            match (self.app_state.input_mode, swap) {
                (InputMode::Fill, false) | (InputMode::Notes, true) => {
                    if let Err(GameError::ConflictingDigit) =
                        self.app_state.game.toggle_digit(pos, digit, policy)
                    {
                        assert_eq!(policy, RuleCheckPolicy::Strict);
                        self.ui_state.conflict_ghost = Some((pos, GhostType::Digit(digit)));
                    }
                }
                (InputMode::Fill, true) | (InputMode::Notes, false) => {
                    if let Err(GameError::ConflictingDigit) =
                        self.app_state.game.toggle_note(pos, digit, policy)
                    {
                        assert_eq!(policy, RuleCheckPolicy::Strict);
                        self.ui_state.conflict_ghost = Some((pos, GhostType::Note(digit)));
                    }
                }
            }
        }
    }

    fn clear_cell(&mut self) {
        if let Some(pos) = self.app_state.selected_cell {
            let _ = self.app_state.game.clear_cell(pos);
        }
    }

    fn apply_action(&mut self, action: Action, ctx: &Context) {
        const DEFAULT_POSITION: Position = Position::new(0, 0);
        self.ui_state.conflict_ghost = None;
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
            Action::ToggleInputMode => {
                self.app_state.input_mode = match self.app_state.input_mode {
                    InputMode::Fill => InputMode::Notes,
                    InputMode::Notes => InputMode::Fill,
                };
            }
            Action::RequestDigit { digit, swap } => {
                self.request_digit(digit, swap);
            }
            Action::ClearCell => {
                self.clear_cell();
            }
            Action::RequestNewGameConfirm => {
                self.ui_state.show_new_game_confirm_dialogue = true;
            }
            Action::NewGame => {
                self.new_game();
            }
            Action::UpdateSettings(settings) => {
                self.app_state.settings = settings;
                self.update_theme(ctx);
            }
        }
    }

    fn update_theme(&self, ctx: &Context) {
        match self.app_state.settings.appearance.theme {
            Theme::Light => {
                ctx.set_visuals(Visuals::light());
            }
            Theme::Dark => {
                ctx.set_visuals(Visuals::dark());
            }
        }
    }

    fn make_grid(&self) -> Array81<GridCell, PositionSemantics> {
        let mut grid = Array81::from_fn(|pos| GridCell {
            content: *self.app_state.game.cell(pos),
            visual_state: GridVisualState::empty(),
            note_visual_state: NoteVisualState::default(),
        });

        if let Some((pos, ghost)) = self.ui_state.conflict_ghost {
            match ghost {
                GhostType::Digit(digit) => {
                    grid[pos].content = CellState::Filled(digit);
                    grid[pos].visual_state.insert(GridVisualState::GHOST);
                }
                GhostType::Note(digit) => {
                    let mut notes = grid[pos].content.as_notes().unwrap_or_default();
                    notes.insert(digit);
                    grid[pos].content = CellState::Notes(notes);
                    grid[pos].note_visual_state.ghost.insert(digit);
                }
            }
        }

        let selected_cell = self.app_state.selected_cell;
        let selected_digit = selected_cell.and_then(|pos| grid[pos].content.as_digit());

        // Highlight the selected cell and its house.
        if let Some(pos) = self.app_state.selected_cell {
            grid[pos].visual_state.insert(GridVisualState::SELECTED);
            for house_pos in pos.house_positions() {
                grid[house_pos]
                    .visual_state
                    .insert(GridVisualState::HOUSE_SELECTED);
            }
        }

        for pos in Position::ALL {
            let cell_digit = grid[pos].content.as_digit();
            let cell_notes = grid[pos].content.as_notes();

            // Highlight conflicts in the same house.
            if let Some(digit) = cell_digit {
                for peer_pos in pos.house_peers() {
                    let peer_digit = grid[peer_pos].content.as_digit();
                    let peer_notes = grid[peer_pos].content.as_notes();
                    if peer_digit == Some(digit) {
                        grid[peer_pos]
                            .visual_state
                            .insert(GridVisualState::CONFLICT);
                    }
                    if peer_notes.is_some_and(|notes| notes.contains(digit)) {
                        grid[peer_pos].note_visual_state.conflict.insert(digit);
                    }
                }
            }

            // Highlight same digits and notes as the selected cell.
            if let Some(digit) = selected_digit {
                if cell_digit == Some(digit) {
                    grid[pos].visual_state.insert(GridVisualState::SAME_DIGIT);
                    for house_pos in pos.house_positions() {
                        grid[house_pos]
                            .visual_state
                            .insert(GridVisualState::HOUSE_SAME_DIGIT);
                    }
                }

                if cell_notes.is_some_and(|notes| notes.contains(digit)) {
                    grid[pos].note_visual_state.same_digit.insert(digit);
                }
            }
        }

        grid
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

        let grid = self.make_grid();
        let game = &self.app_state.game;
        let selected_cell = self.app_state.selected_cell;
        let settings = &self.app_state.settings;
        let notes_mode = self.app_state.input_mode.is_notes();
        let grid_vm = GridViewModel::new(&grid, &settings.assist.highlight);
        let policy = if settings.assist.block_rule_violations {
            RuleCheckPolicy::Strict
        } else {
            RuleCheckPolicy::Permissive
        };
        let decided_digit_count = game.decided_digit_count();
        let digit_capabilities = Array9::from_fn(|digit| {
            let toggle_digit =
                selected_cell.map(|pos| game.toggle_digit_capability(pos, digit, policy));
            let toggle_note =
                selected_cell.map(|pos| game.toggle_note_capability(pos, digit, policy));
            DigitKeyState::new(toggle_digit, toggle_note, decided_digit_count[digit])
        });
        let has_removable_digit = selected_cell.is_some_and(|pos| game.has_removable_digit(pos));
        let keypad_vm = KeypadViewModel::new(digit_capabilities, has_removable_digit, notes_mode);
        let sidebar_vm = SidebarViewModel::new(self.status(), settings);
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
