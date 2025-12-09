use std::{path::Path, error::Error, fs, process};
use crate::cli::run_cli::{Action, Dir};

pub fn get_content_of_current_dir(dir: &mut Dir) -> Result<(), Box<dyn Error>> {
    dir.reset();
    dir.push_content("(back)".to_string());

    let path_str = dir.path.to_string();
    let path = Path::new(&path_str);

    if let Some(parent) = path.parent() {
        if let Some(parent_str) = parent.to_str() {
            dir.change_parent(parent_str.to_string());
        }
    }
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    dir.push_content(name.to_string());
                }
            }
        }
    }
    dir.change_length(dir.content.len());
    Ok(())
}

pub fn update_content(dir: &mut Dir, action: Action) {
    
    if action == Action::UP {
        dir.change_index(-1);
    }
    else if action == Action::DOWN {
        dir.change_index(1);
    }
    else if action == Action::ENTER {
        let index = dir.index;
        if index == 0 {
            dir.change_path(dir.parent_path.to_string());
            //println!("{}", dir.parent());
        }else {
            let path = format!("{}/{}", dir.path, dir.get_content(index).to_string());
            dir.change_path(path);
            println!("New path: {}", dir.path);
        }
        if let Err(e) = get_content_of_current_dir(dir) {
            eprintln!("{e}");
            process::exit(1);
        }
    }
    else{
        println!("NONE");
    }

    update_scroll(dir);

    print!("\x1B[2J\x1B[1;1H");
    println!("Directory: {}\n", dir.path);
    let start = dir.start;
    let end = (start + dir.window_size).min(dir.content.len() - 1);

    for i in start..=end {
        let element = &dir.content[i];
        if i as i32 == dir.index {
            println!("{element} <-");
        } else {
            println!("{element}");
        }
    }
    println!("");
}

pub fn update_scroll(dir: &mut Dir){
    let idx = dir.index as usize;
    if idx < dir.start {
        dir.change_start(idx);
    }
    if idx >= dir.start + dir.window_size {
        dir.change_start(idx - dir.window_size);
    }
}
