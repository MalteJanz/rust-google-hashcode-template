use crate::{DataContext, OutputContext};
use std::io::Write;

pub fn write_data(data_context: DataContext, output_context: OutputContext) {
    let mut writer = data_context.file_context.create_writer();

    // first line
    writer
        .write_all(format!("{}\n", output_context.intersect_schedule_count).as_bytes())
        .unwrap();

    // other lines
    for intersection in output_context.intersect_schedule {
        writer
            .write_all(format!("{}\n", intersection.id).as_bytes())
            .unwrap();

        writer
            .write_all(format!("{}\n", intersection.street_schedule_count).as_bytes())
            .unwrap();

        for street in intersection.street_schedules {
            writer
                .write_all(format!("{} {}\n", street.street_name, street.green_time).as_bytes())
                .unwrap();
        }
    }
}
