use crate::DataContext;
use std::io::Write;

pub fn write_data(data_context: DataContext /* output_data: OutputData */) {
    let mut writer = data_context.file_context.create_writer();

    // write first line
    /*
    writer
        .write_all(format!("{}\n", pizza_deliveries.len()).as_bytes())
        .unwrap();

     */

    // ...
}
