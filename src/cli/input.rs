use crate::cli::run_cli::{Action};

pub fn handle_input(c: char, index: i32, len: usize) -> Action{
    match c {
        'k' => {
            if index - 1 > -1 {
                Action::Up
            }else {
                Action::Empty
            }
        },
        'j' => {
            if index + 1 < len as i32 {
                Action::Down
            }else {
                Action::Empty
            }
        },
        'h' => {
            Action::ShowHiddenFiles
        }
        '\r' | '\n' => {
            Action::Enter
        },
        _ => {
            Action::Empty
        },
    }
}

pub fn handle_sequence(b: char) -> Option<char> {
    match b {
        'A' => Some('k'),
        'B' => Some('j'),
        _ => None,
    }
}
