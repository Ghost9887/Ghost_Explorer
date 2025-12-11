use crate::cli::data::*;
use crate::cli::content::{get_content_of_current_dir, update_content};
use std::{process, io::{self, Read}};

pub fn read_input(mut dir: Dir, mut global: Global) {
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut c = [0u8; 1];

    while handle.read(&mut c).unwrap() == 1 && c[0] != b'q' {
        let char = c[0] as char;
        
        let effective_char = match char {
            '\x1b' => read_escape_sequence(&mut handle),
            _ => Some(char),
        };

        if let Some(char) = effective_char {
            process_char(char, &mut dir, &mut global);
        };
    }
}

fn read_escape_sequence<R: io::Read>(handle: &mut R) -> Option<char> {
    let mut seq = [0u8; 1];

    if handle.read(&mut seq).ok()? != 1 || seq[0] != b'[' {
        return None;
    }
    if handle.read(&mut seq).ok()? != 1 {
        return None;
    }

    handle_sequence(seq[0] as char)
}

fn process_char(c: char, dir: &mut Dir, global: &mut Global) {
    let action = handle_input(c, dir.index, dir.length);
    handle_action(dir, global, action);
    update_content(dir, global);
}


fn handle_input(c: char, index: i32, len: usize) -> Action{
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
        'h' => Action::ShowHiddenFiles,
        'x' => Action::Select,
        'a' => Action::Add,
        'y' => Action::Delete,
        '\r' | '\n' => Action::Enter,
        _ => Action::Empty,
    }
}

pub fn handle_sequence(b: char) -> Option<char> {
    match b {
        'A' => Some('k'),
        'B' => Some('j'),
        _ => None,
    }
}


pub fn handle_action(dir: &mut Dir, global: &mut Global, action: Action) {
    match action {
        Action::Up => {
            dir.change_index(-1);
        },
        Action::Down => {
            dir.change_index(1);
        },
        Action::Enter => {
            if dir.index == 0 {
                dir.change_path(dir.parent_path.to_string());
            }else {
                let path: String;
                if dir.path != "/" {
                    path = format!("{}/{}", dir.path, dir.get_content(dir.index).name.to_string());
                }else {
                    path = format!("/{}", dir.get_content(dir.index).name.to_string());
                }
                dir.change_path(path);
                dir.index = 0;
            }
            if let Err(e) = get_content_of_current_dir(dir, global) {
                eprintln!("{e}");
                process::exit(1);
            }
        },
        Action::ShowHiddenFiles => {
            global.switch_hf();
            if let Err(e) = get_content_of_current_dir(dir, global) {
                eprintln!("{e}");
                process::exit(1);
            }
            if dir.index > (dir.length - 1) as i32 {
                let difference = ((dir.length - 1) as i32) - dir.index;
                dir.change_index(difference);
            }
        },
        Action::Select => {
            if dir.index > 0 {
                let content: &mut Element = dir.get_content_mut(dir.index);  
                content.select();
            }
        },
        Action::Add => {
            global.switch_adding();
        }
        Action::Delete => {
        }
        _ => {}
    }
}

