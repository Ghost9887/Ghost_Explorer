use raw_keys::RawMode;
use crate::cli::content::{get_content_of_current_dir, update_content};
use crate::cli::input::read_input;
use crate::cli::data::{Dir, Global};
use std::process;

pub fn run_cli(){
    
    let mut global = Global::new();
    let mut dir = Dir::new();
    
    let mut raw_mode = RawMode::new().unwrap();

    if let Err(e) = get_content_of_current_dir(&mut dir, &mut global){
        eprintln!("{e}");
        process::exit(1);
    };
    update_content(&mut dir, &mut global);

    if let Err(e) = raw_mode.start(){
        eprintln!("{e}");
        process::exit(1);
    }

    read_input(dir, global);
}


