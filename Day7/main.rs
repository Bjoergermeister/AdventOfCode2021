use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::cmp;

fn calculate(numbers: &Vec<i32>, min: i32, max: i32, use_more_fuel: bool) -> i32 {
    let mut lowest_fuel_cost = std::i32::MAX;
    for i in min..=max {
        let mut current_fuel_cost = 0;
        for j in 0..numbers.len() {            
            if use_more_fuel {
                let normale_fuel_cost = (numbers[j] - i).abs();
                current_fuel_cost += (normale_fuel_cost * (normale_fuel_cost + 1)) / 2;
            } else {
                current_fuel_cost += (numbers[j] - i).abs();
            }         
        }

        if current_fuel_cost < lowest_fuel_cost {
            lowest_fuel_cost = current_fuel_cost;
        }
    }

    return lowest_fuel_cost;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut line: String = String::from(""); 
    let _result = buf_reader.read_line(&mut line);

    let mut numbers: Vec<i32> = Vec::new();
    let mut min = i32::MAX;
    let mut max = 0;
    for number in line.split(",") {
        let number_as_int = number.parse::<i32>().unwrap();
        min = cmp::min(min, number_as_int);
        max = cmp::max(max, number_as_int);
        numbers.push(number_as_int);

    }

    println!("Puzzle 1: {}", calculate(&numbers, min, max, false));
    println!("Puzzle 2: {}", calculate(&numbers, min, max, true));
}