use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::cmp;

fn main() {
    let file = File::open("input.txt").expect("Something went wrong!");
    let reader = BufReader::new(file);
    let mut total_fuel_required = 0;

    for module in reader.lines() {
        let module_weight = module.expect("Something went wrong!").parse::<i32>().unwrap();
        total_fuel_required += calc_fuel_for(module_weight);
    }

    println!("{}", total_fuel_required);
}

fn calc_fuel_for(weight: i32) -> i64 {
    let mut complete = false;
    let mut total_fuel_required = 0;
    let mut remaining_weight: i64 = weight as i64;

    while complete == false {
        let fuel_required = ((remaining_weight / 3) as i64) - 2;
        let capped_fuel_required = cmp::max(fuel_required, 0);
        remaining_weight = capped_fuel_required;

        if capped_fuel_required > 0 {
            total_fuel_required += capped_fuel_required;
        }
        
        if remaining_weight == 0 {
            complete = true;
        }
    }

    total_fuel_required
}
