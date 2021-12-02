use std::fs::File;
use std::io::{BufRead, BufReader};

struct Command {
    command: String,
    value: i32,
}

fn puzzle1(commands: &Vec<Command>) -> i32 {

    let mut depth = 0;
    let mut horizontal_position = 0;

    for command in commands {
    
        if command.command == "up" {
            depth -= command.value;
        }
        else if command.command == "down" {
            depth += command.value;
        }
        else {
            horizontal_position += command.value;
        }
    }

    return horizontal_position * depth;
}

fn puzzle2(commands: &Vec<Command>) -> i32 {
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;

    for command in commands {
    
        if command.command == "up" {
            aim -= command.value;
        }
        else if command.command == "down" {
            aim += command.value;
        }
        else {         
            horizontal_position += command.value;
            depth += command.value * aim;
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

        //Split line in command and value
        let line_string = line.unwrap();
        let parts: Vec<&str> = line_string.split_whitespace().collect();         
        let command = Command { command: parts[0].to_string(), value: parts[1].parse().unwrap() };
        lines.push(command);
    }

    println!("Puzzle 1: {}", puzzle1(&lines));
    println!("Puzzle 2: {}", puzzle2(&lines));
}