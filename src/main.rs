use crate::reader::read_data;
use crate::writer::write_data;
use hashcode_helpers::{create_submission_zip, print_execution_time, FileContext};
use rayon::prelude::*;

mod reader;
mod writer;

#[derive(Debug)]
pub struct DataContext {
    pub file_context: FileContext,
    pub pizza_count: usize,
    pub team_of_two_count: usize,
    pub team_of_three_count: usize,
    pub team_of_four_count: usize,
    pub pizzas: Vec<Pizza>,
}

// input data
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Pizza {
    pub id: usize,
    pub ingredient_count: usize,
    pub ingredients: Vec<String>,
}

// output data
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PizzaDelivery {
    pub team_size: usize,
    pub pizza_ids: Vec<usize>,
}

fn main() {
    println!("Start working on hashcode problem...");

    print_execution_time("main", || {
        let input_files = vec![
            "input/a_example",
            "input/b_little_bit_of_everything.in",
            "input/c_many_ingredients.in",
            "input/d_many_pizzas.in",
            "input/e_many_teams.in",
        ];

        input_files.par_iter().for_each(|file_path| {
            // process each file in parallel
            print_execution_time(file_path, || {
                let data_context = read_data(file_path);
                let data_output = process_data(&data_context);
                write_data(data_context, data_output);
            });
        });

        println!("Zipping source files to output/source.zip");
        print_execution_time("zipping", create_submission_zip);
    });
}

fn process_data(data_context: &DataContext) -> Vec<PizzaDelivery> {
    let mut pizza_deliveries = Vec::new();
    let mut pizza_iter = data_context.pizzas.iter().map(|pizza| pizza.id);
    let pizza_iter = pizza_iter.by_ref();

    // iterate over teamsize
    for team_size in 2..5 {
        let number_of_teams = match team_size {
            2 => data_context.team_of_two_count,
            3 => data_context.team_of_three_count,
            4 => data_context.team_of_four_count,
            _ => {
                return pizza_deliveries;
            }
        };

        // iterate over each team for the given size
        for _ in 0..number_of_teams {
            let pizza_ids = pizza_iter.take(team_size).collect::<Vec<usize>>(); // try to give them (team_size) pizzas

            if pizza_ids.is_empty() {
                return pizza_deliveries;
            }

            pizza_deliveries.push(PizzaDelivery {
                team_size,
                pizza_ids,
            });
        }
    }

    pizza_deliveries
}
