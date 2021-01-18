use crate::{DataContext, PizzaDelivery};
use std::io::Write;

pub fn write_data(data_context: DataContext, pizza_deliveries: Vec<PizzaDelivery>) {
    let mut writer = data_context.file_context.create_writer();

    // write first line
    writer.write_all(format!("{}\n", pizza_deliveries.len()).as_bytes()).unwrap();

    // write other lines
    for pizza_delivery in pizza_deliveries {
        writer.write_all(format!("{} ", pizza_delivery.team_size).as_bytes()).unwrap();

        for pizza_id in &pizza_delivery.pizza_ids {
            // todo: check if the last ' ' space at the end of the line is valid or must be removed.
            writer.write_all(format!("{} ", pizza_id).as_bytes()).unwrap();
        }

        writer.write_all(b"\n").unwrap();
    }
}
