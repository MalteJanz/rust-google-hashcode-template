use crate::{DataContext, Pizza};
use hashcode_helpers::read_input_file;

pub fn read_data(path: &str) -> DataContext {
    let (line_values, file_context) = read_input_file(path);
    let mut line_iter = line_values.into_iter();

    let first_line = line_iter.next().unwrap();

    let pizzas = line_iter
        .enumerate()
        .map(|(i, l)| {
            let mut value_iter = l.into_iter();

            Pizza {
                id: i,
                ingredient_count: value_iter.next().unwrap().parse().unwrap(),
                ingredients: value_iter.collect(),
            }
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
