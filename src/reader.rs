use crate::{DataContext, Pizza};
use hashcode_helpers::read_input_file;

pub fn read_data(path: &str) -> DataContext {
    let (line_values, file_context) = read_input_file(path);
    let mut line_iter = line_values.into_iter();

    let first_line = line_iter.next().unwrap();

    let pizzas = line_iter
        .enumerate()
        .map(|(i, l)| Pizza {
            id: i,
            ingredient_count: l[0].parse().unwrap(),
            ingredients: l.into_iter().skip(1).collect(),
        })
        .collect();

    DataContext {
        file_context,
        pizza_count: first_line[0].parse().unwrap(),
        team_of_two_count: first_line[1].parse().unwrap(),
        team_of_three_count: first_line[2].parse().unwrap(),
        team_of_four_count: first_line[3].parse().unwrap(),
        pizzas,
    }
}
