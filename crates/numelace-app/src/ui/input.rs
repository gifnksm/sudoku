use eframe::egui::{InputState, Key};
use numelace_core::Digit;

use crate::action::{Action, ActionRequestQueue, MoveDirection};

pub fn handle_input(i: &InputState, action_queue: &mut ActionRequestQueue) {
    if i.modifiers.command && i.key_pressed(Key::N) {
        action_queue.request(Action::RequestNewGameConfirm);
    }
    if i.key_pressed(Key::ArrowUp) {
        action_queue.request(Action::MoveSelection(MoveDirection::Up));
    }
    if i.key_pressed(Key::ArrowDown) {
        action_queue.request(Action::MoveSelection(MoveDirection::Down));
    }
    if i.key_pressed(Key::ArrowLeft) {
        action_queue.request(Action::MoveSelection(MoveDirection::Left));
    }
    if i.key_pressed(Key::ArrowRight) {
        action_queue.request(Action::MoveSelection(MoveDirection::Right));
    }
    if i.key_pressed(Key::Escape) {
        action_queue.request(Action::ClearSelection);
    }
    if i.key_pressed(Key::S) {
        action_queue.request(Action::ToggleInputMode);
    }
    let pairs = [
        (Key::Delete, None),
        (Key::Backspace, None),
        (Key::Num1, Some(Digit::D1)),
        (Key::Num2, Some(Digit::D2)),
        (Key::Num3, Some(Digit::D3)),
        (Key::Num4, Some(Digit::D4)),
        (Key::Num5, Some(Digit::D5)),
        (Key::Num6, Some(Digit::D6)),
        (Key::Num7, Some(Digit::D7)),
        (Key::Num8, Some(Digit::D8)),
        (Key::Num9, Some(Digit::D9)),
    ];
    for (key, digit) in pairs {
        if i.key_pressed(key) {
            if let Some(digit) = digit {
                action_queue.request(Action::RequestDigit {
                    digit,
                    swap: i.modifiers.command,
                });
            } else {
                action_queue.request(Action::ClearCell);
            }
        }
    }
}
