use numelace_core::{Digit, Position};
use numelace_game::{Game, RuleCheckPolicy};

#[derive(Debug)]
pub struct AppState {
    pub game: Game,
    pub selected_cell: Option<Position>,
    pub input_mode: InputMode,
    pub settings: Settings,
}

impl AppState {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            selected_cell: None,
            input_mode: InputMode::Fill,
            settings: Settings::default(),
        }
    }

    pub fn rule_check_policy(&self) -> RuleCheckPolicy {
        if self.settings.assist.block_rule_violations {
            RuleCheckPolicy::Strict
        } else {
            RuleCheckPolicy::Permissive
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::IsVariant)]
pub enum InputMode {
    Fill,
    Notes,
}

impl InputMode {
    pub fn toggle(&mut self) {
        *self = match self {
            InputMode::Fill => InputMode::Notes,
            InputMode::Notes => InputMode::Fill,
        }
    }

    pub fn swapped(self, swap: bool) -> Self {
        if swap {
            match self {
                InputMode::Fill => InputMode::Notes,
                InputMode::Notes => InputMode::Fill,
            }
        } else {
            self
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Settings {
    pub assist: AssistSettings,
    pub appearance: AppearanceSettings,
}

#[derive(Debug, Clone)]
pub struct AssistSettings {
    pub block_rule_violations: bool,
    pub highlight: HighlightSettings,
}

impl Default for AssistSettings {
    fn default() -> Self {
        Self {
            block_rule_violations: true,
            highlight: HighlightSettings::default(),
        }
    }
}

#[derive(Debug, Clone)]
#[expect(clippy::struct_excessive_bools)]
pub struct HighlightSettings {
    pub same_digit: bool,
    pub house_selected: bool,
    pub house_same_digit: bool,
    pub conflict: bool,
}

impl Default for HighlightSettings {
    fn default() -> Self {
        Self {
            same_digit: true,
            house_selected: true,
            house_same_digit: true,
            conflict: true,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AppearanceSettings {
    pub theme: Theme,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhostType {
    Digit(Digit),
    Note(Digit),
}

#[derive(Debug, Default)]
pub struct UiState {
    pub show_new_game_confirm_dialogue: bool,
    pub conflict_ghost: Option<(Position, GhostType)>,
}
