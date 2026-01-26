use numelace_core::Position;
use numelace_game::Game;

#[derive(Debug)]
pub struct AppState {
    pub game: Game,
    pub selected_cell: Option<Position>,
    pub settings: Settings,
}

impl AppState {
    pub fn new(game: Game, theme: Theme) -> Self {
        Self {
            game,
            selected_cell: None,
            settings: Settings::new(theme),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub highlight: HighlightSettings,
    pub theme: ThemeSettings,
}

impl Settings {
    pub fn new(theme: Theme) -> Self {
        Self {
            highlight: HighlightSettings::default(),
            theme: ThemeSettings::new(theme),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HighlightSettings {
    pub same_digit: bool,
    pub rcb_selected: bool,
    pub rcb_same_digit: bool,
}

impl Default for HighlightSettings {
    fn default() -> Self {
        Self {
            same_digit: true,
            rcb_selected: true,
            rcb_same_digit: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeSettings {
    pub theme: Theme,
}

impl ThemeSettings {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Default)]
pub struct UiState {
    pub show_new_game_confirm_dialogue: bool,
}
