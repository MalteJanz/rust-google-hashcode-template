use crate::reader::read_data;
use crate::writer::write_data;
use hashcode_helpers::{create_submission_zip, print_execution_time, FileContext};

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
    print_execution_time("main", || {
        println!("Starting hashcode worker");
        let data_context = read_data("input/a_example");
        let data_output = print_execution_time("process_data", || process_data(&data_context));
        write_data(data_context, data_output);

        println!("Zipping source files to output/source.zip");
        create_submission_zip();
    });
}

fn process_data(data_context: &DataContext) -> Vec<PizzaDelivery> {
    let mut pizza_deliveries = Vec::new();
    let mut pizza_iter = data_context.pizzas.iter().map(|pizza| pizza.id);
    let pizza_iter = pizza_iter.by_ref();

    // iterate over teamsize
    for team_size in (2..5).rev() {
        let number_of_teams = match team_size {
            4 => data_context.team_of_four_count,
            3 => data_context.team_of_three_count,
            2 => data_context.team_of_two_count,
            _ => {
                return pizza_deliveries;
            }
        };

        // iterate over each team for the given size
        for _ in 0..number_of_teams {
            let pizza_ids = pizza_iter.take(2).collect::<Vec<usize>>(); // try to give them two pizzas

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
