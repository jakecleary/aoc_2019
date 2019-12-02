use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Something went wrong!");
    let reader = BufReader::new(file);

    let mut total_fuel_required = 0;

    for line in reader.lines() {
        let weight = line.expect("Something went wrong!").parse::<i32>().unwrap();
        let fuel_required = ((weight / 3) as i64) - 2;
        total_fuel_required += fuel_required;
    }

    println!("{}", total_fuel_required);
}
