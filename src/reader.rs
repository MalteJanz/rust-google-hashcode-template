use crate::{DataContext};
use hashcode_helpers::read_input_file;

pub fn read_data(path: &str) -> DataContext {
    let (line_values, file_context) = read_input_file(path);
    let mut line_iter = line_values.into_iter();

    let first_line = line_iter.next().unwrap();

    // ...

    DataContext {
        file_context,
        // ...
    }
}
