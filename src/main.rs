use crate::reader::read_data;
use crate::writer::write_data;
use hashcode_helpers::{create_submission_zip, print_execution_time, FileContext};

mod reader;
mod writer;

#[derive(Debug)]
pub struct DataContext {
    pub file_context: FileContext,
    //...
}

fn main() {
    println!("Start working on hashcode problem...");

    let input_files = vec![
        "input/a_example",
        //"input/...",
    ];

    for file_path in input_files {
        let data_context = read_data(file_path);
        let data_output = process_data(&data_context);
        write_data(data_context /*, data_output*/);
    }

    println!("Zipping source files to output/source.zip");
    create_submission_zip();
}

pub fn process_data(data_context: &DataContext) /* -> OutputData */
{
    // ...
}
