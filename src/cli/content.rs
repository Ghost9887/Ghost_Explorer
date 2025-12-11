use std::{path::Path, error::Error, fs, process};
use crate::cli::data::*;
use simply_colored::*;

pub fn get_content_of_current_dir(dir: &mut Dir, global: &mut Global) -> Result<(), Box<dyn Error>> {

    dir.reset();
    dir.push_content(Element::new(String::from("(back)"), Type::Return));

    let path_str = dir.path.to_string();
    let path = Path::new(&path_str);

    if let Some(parent) = path.parent() {
        if let Some(parent_str) = parent.to_str() {
            dir.change_parent(parent_str.to_string());
        }
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(name) = entry.file_name().to_str() {
                let path = format!("{}/{}", dir.path, name);
                let element_type: Type = get_type(&path, name);
                if should_push(&element_type, global) {
                    dir.push_content(Element::new(name.to_string(), element_type)); 
                }
            }
        }
    }
    dir.change_length(dir.content.len());
    Ok(())
}

pub fn update_content(dir: &mut Dir, global: &mut Global, action: Action) {
    
    handle_action(dir, global, action);
    update_scroll(dir, global);

    print!("\x1B[2J\x1B[1;1H");
    println!("Directory: {}\n", dir.path);
    let start = global.start;
    let end = (start + global.window_size).min(dir.length - 1);

    for i in start..=end {
        let element = &dir.content[i];
        if i as i32 == dir.index {
            match element.element_type {
                Type::HiddenFile => {
                    if !element.selected{
                        println!("{RED}{}{RESET} <-", element.name);
                    }
                    else {
                        println!("{GREEN}{}{RESET} * <-", element.name);
                    }
                },
                Type::Directory => {
                    if !element.selected {
                        println!("{BLUE}{}{RESET} <-", element.name);
                    }
                    else {
                        println!("{GREEN}{}{RESET} * <- ", element.name);
                    }
                },
                _ => {
                    if !element.selected {
                        println!("{} <-", element.name);
                    }
                    else {
                        println!("{GREEN}{}{RESET} * <-", element.name);
                    }
                }    
            }
            
        } else {
            match element.element_type {
                Type::HiddenFile => {
                    if !element.selected {
                        println!("{RED}{}{RESET}", element.name);
                    }else {
                        println!("{GREEN}{}{RESET} *", element.name);
                    }
                },
                Type::Directory => {
                    if !element.selected {
                        println!("{BLUE}{}{RESET}", element.name);
                    }else {
                        println!("{GREEN}{}{RESET} *" , element.name);
                    }
                }
                _ => {
                    if !element.selected {
                        println!("{}", element.name);
                    }
                    else {
                        println!("{GREEN}{}{RESET} *", element.name);
                    }
                }    
            }
        }
    }
    println!("");
}

pub fn update_scroll(dir: &mut Dir, global: &mut Global){
    let idx = dir.index as usize;
    if idx < global.start {
        global.change_start(idx);
    }
    if idx >= global.start + global.window_size {
        global.change_start(idx - global.window_size);
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
        Action::AddFile => {
            
        }
        Action::AddDirectory => {

        }
        Action::Delete => {

        }
        _ => {}
    }
}

fn get_type(path: &str, name: &str) -> Type {
    let metadata = fs::metadata(path).unwrap();
    if name.starts_with('.') {
        Type::HiddenFile
    } else if metadata.is_file() {
        Type::File
    } else if metadata.is_dir() {
        Type::Directory
    } else {
        Type::Other
    }
}

fn should_push(element_type: &Type, global: &mut Global) -> bool {
    match element_type {
        Type::HiddenFile => global.hidden_files,
        _ => true,
    }
}
