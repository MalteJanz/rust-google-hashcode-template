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
        let mut data_context = read_data("input/a_example");
        let data_output = print_execution_time("process_data", || process_data(&mut data_context));
        write_data(data_context, data_output);

        println!("Zipping source files to output/source.zip");
        create_submission_zip();
    });
}

fn process_data(data_context: &mut DataContext) -> Vec<PizzaDelivery> {
    for pizza in &data_context.pizzas {
        println!(
            "pizza: id={}, ingredient_coutn={} ingredients={:?}",
            pizza.id, pizza.ingredient_count, pizza.ingredients
        );
    }

    // Todo: implement processing logic
    let pizza_deliveries = vec![
        PizzaDelivery {
            team_size: 2,
            pizza_ids: vec![0],
        },
        PizzaDelivery {
            team_size: 3,
            pizza_ids: vec![1, 2],
        },
        PizzaDelivery {
            team_size: 3,
            pizza_ids: vec![3, 4],
        },
    ];

    pizza_deliveries
}
