use crate::{DataContext, PizzaDelivery};
use std::io::Write;

pub fn write_data(data_context: DataContext, pizza_deliveries: Vec<PizzaDelivery>) {
    let mut writer = data_context.file_context.create_writer();

    // write first line
    writer
        .write_all(format!("{}\n", pizza_deliveries.len()).as_bytes())
        .unwrap();

    // write other lines
    for pizza_delivery in pizza_deliveries {
        writer
            .write_all(pizza_delivery.team_size.to_string().as_bytes())
            .unwrap();

        let ids_concatenated = pizza_delivery
            .pizza_ids
            .iter()
            .fold(String::new(), |a, b| format!("{} {}", a, b));

        writer.write_all(ids_concatenated.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
    }
}
