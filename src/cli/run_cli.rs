use std::{process, io::{self, Read}};
use raw_keys::RawMode;
use crate::cli::content::{get_content_of_current_dir, update_content};
use crate::cli::input::{handle_input, handle_sequence, handle_action};
use crate::cli::data::{Dir, Global};

pub fn run_cli(){
    
    let mut global = Global::new();
    let mut dir = Dir::new();
    
    let mut raw_mode = RawMode::new().unwrap();

        let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut c = [0u8; 1];

    if let Err(e) = get_content_of_current_dir(&mut dir, &mut global){
        eprintln!("{e}");
        process::exit(1);
    };

    update_content(&mut dir, &mut global);

if let Err(e) = raw_mode.start(){
        eprintln!("{e}");
        process::exit(1);
    }
    while handle.read(&mut c).unwrap() == 1 && c[0] != b'q' {
        let char = c[0] as char;

        if char == '\x1b' {
            let mut seq1 = [0u8; 1];
            if handle.read(&mut seq1).unwrap() != 1 {
                continue;
            }
            if seq1[0] != b'[' {
                continue;
            }
            let mut seq2 = [0u8; 1];
            if handle.read(&mut seq2).unwrap() != 1 {
                continue;
            }

            let char2 = seq2[0] as char;
            let char_equivelant = handle_sequence(char2);

            match char_equivelant {
                Some(c) => {
                    let action = handle_input(c, dir.index, dir.length);
                    handle_action(&mut dir, &mut global, action);
                    update_content(&mut dir, &mut global);
                    continue;
                    },
                    None => continue,
                };
            }
            let action = handle_input(char, dir.index, dir.length);
            handle_action(&mut dir, &mut global, action);
            update_content(&mut dir, &mut global);
    }
}


