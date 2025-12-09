use std::{process, io::{self, Read}};
use raw_keys::RawMode;
use crate::cli::content::{get_content_of_current_dir, show_content};
use crate::cli::input::{handle_input};

pub fn run_cli(){
    let global_path = String::from(".");
    let mut content: Vec<String> = Vec::new();
    if let Err(e) = get_content_of_current_dir(&global_path, &mut content){
        eprintln!("{e}");
        process::exit(1);
    }

    let _raw_mode = RawMode::new().unwrap();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut c = [0u8; 1];

    let mut index = 0;
    show_content(&content, index);

    while handle.read(&mut c).unwrap() == 1 && c[0] != b'q' {
        //println!("{}", c[0] as char);
        let new_index = handle_input(c[0] as char);
        if new_index < 0 && index == 0 {
            continue;
        }
        else if new_index > 0 && index == (content.len() - 1) as i32 {
            continue;
        }
        else {
            index += new_index;
            show_content(&content, index);
        }
    }
}

