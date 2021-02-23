use crate::reader::read_data;
use crate::writer::write_data;
use hashcode_helpers::{create_submission_zip, print_execution_time, FileContext};
use rayon::prelude::*;

mod reader;
mod writer;

#[derive(Debug)]
pub struct DataContext {
    pub file_context: FileContext,
    //...
}

fn main() {
    println!("Start working on hashcode problem...");

    print_execution_time("main", || {
        let input_files = vec![
            "input/a_example",
            //"input/...",
        ];

        input_files.into_par_iter().for_each(|file_path| {
            // process each file in parallel
            print_execution_time(file_path, || {
                let data_context = read_data(file_path);
                let data_output = process_data(&data_context);
                write_data(data_context /*, data_output*/);
            });
        });

        println!("Zipping source files to output/source.zip");
        print_execution_time("zipping", create_submission_zip);
    });
}

pub fn process_data(data_context: &DataContext) /* -> OutputData */
{
    // ...
}
