use crate::reader::read_data;
use crate::writer::write_data;
use hashcode_helpers::{create_submission_zip, print_execution_time, FileContext};
use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::HashMap;

mod reader;
mod writer;

// input
#[derive(Debug)]
pub struct DataContext {
    pub file_context: FileContext,
    pub simulation_time: usize,
    pub intersection_count: usize,
    pub street_count: usize,
    pub car_count: usize,
    pub bonus: usize,
    pub streets: Vec<Street>,
    pub cars: Vec<Car>,
}

#[derive(Debug)]
pub struct Street {
    pub start_intersection: usize,
    pub end_intersection: usize,
    pub name: String,
    pub time: usize,
}

#[derive(Debug)]
pub struct Car {
    pub street_count: usize,
    pub street_names: Vec<String>,
}

// output
#[derive(Debug)]
pub struct OutputContext {
    pub intersect_schedule_count: usize,
    pub intersect_schedule: Vec<IntersectSchedule>,
}

#[derive(Debug)]
pub struct IntersectSchedule {
    pub id: usize,
    pub street_schedule_count: usize,
    pub street_schedules: Vec<StreetSchedule>,
}

#[derive(Debug)]
pub struct StreetSchedule {
    pub street_name: String,
    pub green_time: usize,
    pub intersection_visitor_factor: f64,
}

fn main() {
    println!("Start working on hashcode problem...");

    print_execution_time("main", || {
        let input_files = vec![
            "input/a.txt",
            "input/b.txt",
            "input/c.txt",
            "input/d.txt",
            "input/e.txt",
            "input/f.txt",
        ];

        input_files.into_par_iter().for_each(|file_path| {
            // process each file in parallel
            print_execution_time(file_path, || {
                let data_context = read_data(file_path);
                //println!("{:#?}", data_context);
                let data_output = process_data(&data_context);
                write_data(data_context, data_output);
            });
        });

        println!("Zipping source files to output/source.zip");
        print_execution_time("zipping", create_submission_zip);
    });
}

pub fn process_data(data_context: &DataContext) -> OutputContext {
    // intersection id with incoming streets into the intersection
    let mut intersections: HashMap<usize, Vec<&Street>> = HashMap::new();
    // street name lookup table
    let mut streets: HashMap<String, &Street> = HashMap::new();

    for street in &data_context.streets {
        intersections
            .entry(street.end_intersection)
            .or_insert_with(Vec::new)
            .push(street);
        streets.insert(street.name.clone(), street);
    }

    // count the streets in the car paths
    let mut used_streets: HashMap<String, usize> = HashMap::new();
    for car in &data_context.cars {
        for street_name in &car.street_names {
            *used_streets.entry(street_name.clone()).or_insert(0) += 1;
        }
    }

    let mut intersection_schedules = Vec::new();
    for (intersection_id, incoming_streets) in &intersections {
        let mut visitor_sum = 0;
        for street in incoming_streets {
            visitor_sum += used_streets.get(&street.name).unwrap_or(&0);
        }

        //println!("Kreuzung {:?} visitorSum= {:?}", intersection_id, visitor_sum);

        let mut schedules = Vec::new();
        for street in incoming_streets {
            if !used_streets.contains_key(&street.name) {
                continue;
            }

            let street_traffic_sum = *used_streets.get(&street.name).expect("street not found");
            // 1 <- all traffic drives through this one street
            // 0 <- no traffic
            let traffic_light_used_factor = street_traffic_sum as f64 / visitor_sum as f64;

            //println!("Street '{:?}' traffic sum = {:?} factor = {:?}", street.name, street_traffic_sum, traffic_light_used_factor);

            schedules.push(StreetSchedule {
                street_name: street.name.clone(),

                // very good for datasets A, B, D
                //green_time: max(1, (street_traffic_sum as f64 / data_context.simulation_time as f64) as usize),

                // very good for dataset C, F!
                //green_time: min(data_context.simulation_time, max(1, f64::ceil(traffic_light_used_factor * incoming_streets.len() as f64) as usize)),

                // best result for dataset F! + Overall best
                green_time: min(
                    data_context.simulation_time,
                    max(1, (traffic_light_used_factor * 10.0) as usize),
                ),

                // best result for dataset E!
                //green_time: min(data_context.simulation_time, max(1, f64::ceil(traffic_light_used_factor * incoming_streets.len() as f64) as usize)),

                // best result for dataset A (only 1k increase)
                //green_time: min(data_context.simulation_time, max(1, incoming_streets.len())),
                intersection_visitor_factor: traffic_light_used_factor as f64,
            });
        }

        if schedules.is_empty() {
            continue;
        }

        schedules.sort_by(|schedule_a, schedule_b| {
            schedule_a
                .intersection_visitor_factor
                .partial_cmp(&schedule_b.intersection_visitor_factor)
                .expect("not a number sorting")
        });

        /*
        if schedules.iter().any(|schedule| schedule.intersection_visitor_factor > 0.9) {
            schedules = schedules.into_iter().filter(|schedule| schedule.intersection_visitor_factor >= 0.9).collect();
        }
        */

        intersection_schedules.push(IntersectSchedule {
            id: *intersection_id,
            street_schedule_count: schedules.len(),
            street_schedules: schedules,
        })
    }

    OutputContext {
        intersect_schedule_count: intersection_schedules.len(),
        intersect_schedule: intersection_schedules,
    }
}
