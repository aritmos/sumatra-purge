//! Program to filter out non-existent file-state records from SumatraPDF.
//! New settings are written into `SumatraPDF-settings-filtered.txt`

fn main() {
    let settings_filepath = std::env::args().nth(1).unwrap_or({
        // try to find SumatraPDF using PATH, else try current folder
        let where_sumatrapdf = std::process::Command::new("where SumatraPDF").output();
        match where_sumatrapdf {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
            _ => "SumatraPDF-settings.txt".to_string(),
        }
    });

    filter_settings(settings_filepath);
}

fn filter_settings(settings_filepath: String) {
    let file = std::fs::read_to_string(settings_filepath.clone()).expect("Settings file not found");

    let first_split_delimiter = "FileStates [\r\n";
    let second_split_delimiter = "\r\n]\r\n";
    let (pre, remainder) = file
        .split_once(first_split_delimiter)
        .expect("First split failed");

    let (file_states, post) = remainder
        .split_once(second_split_delimiter)
        .expect("Second split failed");

    // file state str: "[\r\n\t\tFilePath = <FilePath>\r\n ... \r\n]\r\n\t"
    let file_state_delimiter = "\r\n\t]\r\n\t";
    let file_states = file_states.split_inclusive(file_state_delimiter);
    let filtered_file_states = file_states.into_iter().filter(exists);

    // write out the filtered settings
    let mut out_string = String::new();
    out_string += pre;
    out_string += first_split_delimiter;
    filtered_file_states.for_each(|s| out_string += s);
    out_string += second_split_delimiter;
    out_string += post;

    let settings_filepath_bak = settings_filepath.clone().replace(".txt", ".bak");
    std::fs::rename(settings_filepath.clone(), settings_filepath_bak)
        .expect("Could not rename settings file in order to create backup.");

    std::fs::write(settings_filepath, out_string)
        .expect("Could not write filtered settings into file.");
}

fn exists(file_state: &&str) -> bool {
    let prefix = "[\r\n\t\tFilePath = ";
    let line_break = "\r\n";
    let file_path = file_state
        .strip_prefix(prefix)
        .unwrap()
        .split(line_break)
        .next()
        .unwrap();
    std::path::Path::new(file_path).exists()
}
