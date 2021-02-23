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
    println!("Starting hashcode worker");
    let data_context = read_data("input/a_example");
    //let data_output = process_data(&data_context);
    write_data(data_context/*, data_output*/);

    println!("Zipping source files to output/source.zip");
    create_submission_zip();
}
