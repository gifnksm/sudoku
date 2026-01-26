use serde::{Deserialize, Serialize};

use std::fmt::Write;

use numelace_core::{DigitGrid, DigitGridParseError, Position, PositionNewError};
use numelace_game::{CellState, Game, GameError};

use crate::state::{AppState, HighlightSettings, Settings, Theme, ThemeSettings};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersistedState {
    game: GameDto,
    selected_cell: Option<PositionDto>,
    settings: SettingsDto,
}

impl From<&AppState> for PersistedState {
    fn from(value: &AppState) -> Self {
        Self {
            game: GameDto::from(&value.game),
            selected_cell: value.selected_cell.map(PositionDto::from),
            settings: SettingsDto::from(&value.settings),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum AppStateConversionError {
    #[display("failed to parse game data: {_0}")]
    GameParse(DigitGridParseError),
    #[display("failed to apply saved digit: {_0}")]
    GameRestore(GameError),
    #[display("failed to construct selected position: {_0}")]
    PositionNew(PositionNewError),
}

impl TryFrom<PersistedState> for AppState {
    type Error = AppStateConversionError;

    fn try_from(value: PersistedState) -> Result<Self, Self::Error> {
        Ok(Self {
            game: value.game.try_into()?,
            selected_cell: value.selected_cell.map(Position::try_from).transpose()?,
            settings: value.settings.into(),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameDto {
    problem: String,
    filled: String,
}

impl From<&Game> for GameDto {
    fn from(value: &Game) -> Self {
        let mut problem = String::with_capacity(81);
        let mut filled = String::with_capacity(81);

        for pos in Position::ALL {
            match value.cell(pos) {
                CellState::Given(digit) => {
                    let _ = write!(problem, "{digit}");
                    filled.push('.');
                }
                CellState::Filled(digit) => {
                    problem.push('.');
                    let _ = write!(filled, "{digit}");
                }
                CellState::Empty => {
                    problem.push('.');
                    filled.push('.');
                }
            }
        }

        Self { problem, filled }
    }
}

impl From<Game> for GameDto {
    fn from(value: Game) -> Self {
        GameDto::from(&value)
    }
}

impl TryFrom<GameDto> for Game {
    type Error = AppStateConversionError;

    fn try_from(value: GameDto) -> Result<Self, Self::Error> {
        let problem: DigitGrid = value.problem.parse()?;
        let filled: DigitGrid = value.filled.parse()?;
        Ok(Game::from_problem_filled(&problem, &filled)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PositionDto {
    x: u8,
    y: u8,
}

impl From<Position> for PositionDto {
    fn from(value: Position) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
        }
    }
}

impl TryFrom<PositionDto> for Position {
    type Error = PositionNewError;

    fn try_from(value: PositionDto) -> Result<Self, Self::Error> {
        Position::try_new(value.x, value.y)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsDto {
    highlight: HighlightSettingsDto,
    theme: ThemeSettingsDto,
}

impl From<&Settings> for SettingsDto {
    fn from(value: &Settings) -> Self {
        Self {
            highlight: HighlightSettingsDto::from(&value.highlight),
            theme: ThemeSettingsDto::from(&value.theme),
        }
    }
}

impl From<SettingsDto> for Settings {
    fn from(value: SettingsDto) -> Self {
        Self {
            highlight: value.highlight.into(),
            theme: value.theme.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HighlightSettingsDto {
    pub same_digit: bool,
    pub rcb_selected: bool,
    pub rcb_same_digit: bool,
}

impl From<&HighlightSettings> for HighlightSettingsDto {
    fn from(value: &HighlightSettings) -> Self {
        Self {
            same_digit: value.same_digit,
            rcb_selected: value.rcb_selected,
            rcb_same_digit: value.rcb_same_digit,
        }
    }
}

impl From<HighlightSettingsDto> for HighlightSettings {
    fn from(value: HighlightSettingsDto) -> Self {
        Self {
            same_digit: value.same_digit,
            rcb_selected: value.rcb_selected,
            rcb_same_digit: value.rcb_same_digit,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeSettingsDto {
    pub theme: ThemeDto,
}

impl From<&ThemeSettings> for ThemeSettingsDto {
    fn from(value: &ThemeSettings) -> Self {
        Self {
            theme: value.theme.into(),
        }
    }
}

impl From<ThemeSettingsDto> for ThemeSettings {
    fn from(value: ThemeSettingsDto) -> Self {
        Self {
            theme: value.theme.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ThemeDto {
    Light,
    Dark,
}

impl From<ThemeDto> for Theme {
    fn from(value: ThemeDto) -> Self {
        match value {
            ThemeDto::Light => Theme::Light,
            ThemeDto::Dark => Theme::Dark,
        }
    }
}

impl From<Theme> for ThemeDto {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Light => ThemeDto::Light,
            Theme::Dark => ThemeDto::Dark,
        }
    }
}
