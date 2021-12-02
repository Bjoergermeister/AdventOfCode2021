use std::fs::File;
use std::io::{BufRead, BufReader};

fn puzzle1(lines: Vec<String>) -> i32 {

    let mut depth = 0;
    let mut horizontal_position = 0;

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect(); 
        let command = parts[0];
        let value: i32 = parts[1].parse().unwrap();
    
        if command == "up" {
            depth -= value;
        }
        else if command == "down" {
            depth += value;
        }
        else {
            horizontal_position += value;
        }
    }

    return horizontal_position * depth;
}

fn main()
{
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    println!("Puzzle 1: {}", puzzle1(lines));
}