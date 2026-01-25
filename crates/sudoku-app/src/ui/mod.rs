use sudoku_core::{Digit, Position};

use crate::app::HighlightConfig;

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
    SetDigit(Digit),
    RemoveDigit,
    NewGame,
    UpdateHighlightConfig(HighlightConfig),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
