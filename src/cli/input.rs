use crate::cli::run_cli::{Action};

pub fn handle_input(c: char, index: i32, len: usize) -> Action{
    match c {
        'k' => {
            if index - 1 > -1 {
                Action::UP
            }else {
                Action::NONE
            }
        },
        'j' => {
            if index + 1 < len as i32 {
                Action::DOWN
            }else {
                Action::NONE
            }
        },
        '\r' | '\n' => {
            Action::ENTER
        },
        _ => {
            Action::NONE
        },
    }
}

pub fn handle_sequence(b: char) -> char {
    match b {
        'A' => 'k',
        'B' => 'j',
        _ => ' ',
    }
}
