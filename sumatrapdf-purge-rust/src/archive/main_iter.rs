/// An iterator over a slice `&[T]` that returns subslices `&[T]`
/// based on a predicate function that searches for a closing delimiter.
/// The result of the predicate function is included in the returned slice.
///
/// NOTE:
/// - Current implementation is set to panic if a the predicate function can't find the closing
/// delimiter.
///
/// ```rust
/// let s = &[1, 2, 3, 4, 5, 6];
/// let predicate = Box::new(|x| x % 3 == 0);
/// let mut iter = ChunkedSliceIterator::new(s, predicate).into_iter();
/// assert_eq!(iter.next(), Some(&[1,2,3]));
/// assert_eq!(iter.next(), Some(&[4,5,6]));
/// assert_eq!(iter.next(), None);
/// ```
struct ChunkedSliceIterator<'a, T> {
    data: &'a [T],
    end_delimiter_func: Box<dyn Fn(&T) -> bool>,
}

impl<'a, T> ChunkedSliceIterator<'a, T> {
    fn new(data: &'a [T], end_delimiter_func: Box<dyn Fn(&T) -> bool>) -> Self {
        Self {
            data,
            end_delimiter_func,
        }
    }
}

impl<'a, T> Iterator for ChunkedSliceIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        if let Some(index) = self.data.iter().position(|x| (self.end_delimiter_func)(x)) {
            let subslice = &self.data[..index + 1];
            self.data = &self.data[(index + 1)..];
            Some(subslice)
        } else {
            // There are multiple ways of handling a non-existent end delimiter:

            // A. return remaining elements

            // let subslice = self.data;
            // self.data = &[];
            // Some(subslice)

            // B. return None

            // None

            // C. Panic

            panic!("No end delimiter found")
        }
    }
}

pub fn filter_settings_iter(settings_filepath: String) {
    let file = std::fs::read_to_string(settings_filepath.clone()).expect("Settings file not found");
    let mut lines = file.lines().collect::<Vec<&str>>();

    let filestate_start_idx = {
        let idx = lines.iter().position(|&l| l.starts_with("FileStates"));
        idx.expect("FileStates section not found, aborting.") + 1
    };
    let filestate_end_idx = {
        let idx = lines[filestate_start_idx..].iter().position(|&l| l == "]");
        idx.expect("FileStates section end not found, aborting.") + filestate_start_idx
    };

    // Done in an inner block because we use an immutable borrow of `lines`
    // and we want a mutable borrow later on to stitch back the existing files.
    let existing_files_flattened = {
        let filestate_lines = &lines[filestate_start_idx..filestate_end_idx];

        let filestate_iter =
            ChunkedSliceIterator::new(filestate_lines, Box::new(|l| l.starts_with("\t]")));

        fn get_file_path<'a>(filestate: &'a [&'a str]) -> &'a str {
            filestate[1]
                .split_whitespace()
                .nth(2)
                .expect("No filename found")
        }

        // only keep existing files

        let mut existing_files = Vec::new();

        for filestate in filestate_iter {
            let file_path = get_file_path(filestate);
            if std::path::Path::new(file_path).exists() {
                existing_files.push(filestate);
            }
        }

        let existing_files_flattened = existing_files
            .iter()
            .flat_map(|s| s.iter().cloned())
            .collect::<Vec<&str>>();

        existing_files_flattened
    };

    // back up the old settings and write the filtered settings to file

    let settings_filepath_bak = settings_filepath.clone().replace(".txt", ".bak");
    std::fs::rename(settings_filepath.clone(), settings_filepath_bak)
        .expect("Could not rename settings file in order to create backup.");

    lines.splice(
        filestate_start_idx..filestate_end_idx,
        existing_files_flattened,
    );
    let out_buffer = lines.join("\r\n");

    std::fs::write(settings_filepath, out_buffer)
        .expect("Could not write filtered settings into file.");
}
