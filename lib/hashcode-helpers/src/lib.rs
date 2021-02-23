use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

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

        BufWriter::new(file)
    }
}

/// reads an input file and converts it into a list of lines
/// (each is also a list of Strings, that were split by whitespace in each line).
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
        .map(|l| l.split_ascii_whitespace().map(|s| s.to_owned()).collect())
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
    Command::new("zip")
        .arg("-r")
        .arg("-FS") //sync contents to file system (remove old files, add new file, override changes)
        .arg("output/source.zip")
        // add files / dirs to zip
        .arg("lib")
        .arg("src")
        .arg("Cargo.toml")
        .arg("Cargo.lock")
        .arg(".gitignore")
        .arg("LICENSE")
        .output()
        .expect("failed to create zip file of source code");
}

/// Measures time of execution of the clousure and prints it with the given name to the CLI.
/// returns from the closure are also returned from this function.
pub fn print_execution_time<R, F>(name: &str, fnc: F) -> R
where
    F: FnOnce() -> R,
{
    println!("ExecutionTime[{}] Starting execution now...", name);
    let now = Instant::now();
    let return_value = fnc();
    let elapsed = now.elapsed();
    println!("ExecutionTime[{}] Elapsed: {:.2?}", name, elapsed);

    return_value
}
