use numelace_core::{Digit, Position};

use crate::state::{HighlightSettings, ThemeSettings};

pub mod dialogs;
pub mod game_screen;
pub mod grid;
pub mod input;
pub mod keypad;
pub mod sidebar;

#[derive(Debug, Clone)]
pub enum Action {
    SelectCell(Position),
    ClearSelection,
    MoveSelection(MoveDirection),
    ToggleInputMode,
    RequestDigit { digit: Digit, swap: bool },
    ClearCell,
    RequestNewGameConfirm,
    NewGame,
    UpdateHighlightSettings(HighlightSettings),
    UpdateThemeSettings(ThemeSettings),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
