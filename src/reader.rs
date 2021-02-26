use crate::{Car, DataContext, Street};
use hashcode_helpers::read_input_file;

pub fn read_data(path: &str) -> DataContext {
    let (line_values, file_context) = read_input_file(path);
    let mut line_iter = line_values.into_iter();

    let first_line = line_iter.next().unwrap();
    let street_count = first_line[2].parse().unwrap();
    let car_count = first_line[3].parse().unwrap();

    let line_iter = line_iter.by_ref();

    let streets = line_iter
        .take(street_count)
        .map(|line| {
            let mut street_iter = line.into_iter();

            Street {
                start_intersection: street_iter.next().unwrap().parse().unwrap(),
                end_intersection: street_iter.next().unwrap().parse().unwrap(),
                name: street_iter.next().unwrap(),
                time: street_iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let cars = line_iter
        .map(|line| {
            let mut car_iter = line.into_iter();

            Car {
                street_count: car_iter.next().unwrap().parse().unwrap(),
                street_names: car_iter.collect(),
            }
        })
        .collect();

    DataContext {
        file_context,
        simulation_time: first_line[0].parse().unwrap(),
        intersection_count: first_line[1].parse().unwrap(),
        street_count,
        car_count,
        bonus: first_line[4].parse().unwrap(),
        streets,
        cars,
    }
}
