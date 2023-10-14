use std::process::Command;

fn main() {
    let file = include_str!("settings.txt");
    let mut lines = file.lines();
    let mut pre_filestate_settings: Vec<String> = Vec::new();
    loop {
        let line = lines.next().unwrap();
        pre_filestate_settings.push(line.to_string());
        if line.starts_with("FileStates [") {
            break;
        }
    }

    let mut file_states: Vec<Vec<String>> = Vec::new();
    let mut temp: Vec<String> = Vec::new();
    for line in lines {
        temp.push(line.to_string());
        if line.starts_with("	]") {
            file_states.push(temp.clone());
            temp = Vec::new();
        }
    }

    println!("Total: {} files", file_states.len());

    let filtered_file_states: Vec<Vec<String>> = file_states.into_iter().filter(exists).collect();

    println!("Total: {} existing files", filtered_file_states.len());
}

fn exists(file_state: &Vec<String>) -> bool {
    let Some(windows_file_path) = &file_state[1].trim().strip_prefix("FilePath = ") else {
        return false;
    };

    let Ok(child) = Command::new(format!("IF EXIST {windows_file_path} ECHO exists")).spawn() else {
        return false;
    };

    child.stdout.is_some()
}
