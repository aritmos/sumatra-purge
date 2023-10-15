//! Program to filter out non-existent file-state records from SumatraPDF.
//! New settings are written into `SumatraPDF-settings-filtered.txt`

fn main() {
    let mut args = std::env::args().skip(1);

    let default_settings_filepath = "SumatraPDF-settings.txt".to_string();
    let settings_filepath = args.next().unwrap_or(default_settings_filepath);

    let default_output_filepath = "SumatraPDF-settings-filtered.txt".to_string();
    let output_filepath = args.next().unwrap_or(default_output_filepath);

    filter_settings(settings_filepath, output_filepath);
}

fn filter_settings(settings_filepath: String, output_filepath: String) {
    let file = std::fs::read_to_string(settings_filepath).expect("Settings file not found");

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

    std::fs::write(output_filepath, out_string).expect("Unable to write data to output file");
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
