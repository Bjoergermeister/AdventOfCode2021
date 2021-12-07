use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::cmp;

fn puzzle1(numbers: &Vec<i32>) -> i32 {
    let mut min = numbers[0];
    let mut max = numbers[0];

    //Find min and max number
    for i in 0..numbers.len() {
        min = cmp::min(min, numbers[i]);
        max = cmp::max(max, numbers[i]);
    }

    let mut lowest_fuel_cost = 1000000;
    for i in min..=max {
        let mut current_fuel_cost = 0;
        for j in 0..numbers.len() {
            current_fuel_cost += (numbers[j] - i).abs();
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
    for number in line.split(",") {
        numbers.push(number.parse::<i32>().unwrap());
    }

    println!("Puzzle 1: {}", puzzle1(&numbers));

}