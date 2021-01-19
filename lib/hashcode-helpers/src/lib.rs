use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default)]
pub struct FileContext {
    /// The name of the file that is being processed.
    /// Does not contain the file extension (only the name / stem).
    pub name: OsString,
}

impl FileContext {
    /// Creates a buffered writer to the corresponding file in the ./output directory.
    pub fn create_writer(&self) -> BufWriter<File> {
        let mut path = PathBuf::from("output");
        path.push(&self.name);
        let file = File::create(path).expect("can't open file in output/... to write to.");
        let writer = BufWriter::new(file);
        writer
    }
}

/// reads an input file and converts it into a list of lines
/// (each is also a list of Strings, that were splitted by whitespace in each line).
/// Also returns a [FileContext] which can be used to keep track of the data name and write data back into a file.
pub fn read_input_file(path: &str) -> (Vec<Vec<String>>, FileContext) {
    let path = Path::new(path);
    let file_stem = path
        .file_stem()
        .expect("can't get file name from input file path.");
    let file = File::open(path).expect("can't read input file.");
    let reader = BufReader::new(file);

    let line_values = reader
        .lines()
        .map(|l| l.expect("invalid line in input file."))
        .map(|l| l.split_whitespace().map(|s| s.to_owned()).collect())
        .collect();

    let file_context = FileContext {
        name: file_stem.to_owned(),
    };

    (line_values, file_context)
}

/// Zips all source files, that can then be submitted to google hashcode.
/// Depends on the "zip" linux command. Other operating systems or systems that don't have "zip"
/// installed may not work with this.
pub fn create_submission_zip() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("Zipping source files to output/source{}.zip", timestamp);

    Command::new("zip")
        .arg("-r")
        .arg(format!("output/source{}.zip", timestamp))
        .arg("lib")
        .arg("src")
        .arg("Cargo.toml")
        .arg("Cargo.lock")
        .arg(".gitignore")
        .arg("LICENSE")
        .output()
        .expect("failed to create zip file of source code");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
