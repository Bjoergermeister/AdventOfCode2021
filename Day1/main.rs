use std::fs::File;
use std::io::{self, prelude::*};

fn puzzle1(numbers: &Vec<i32>) -> io::Result<i32> {
    let mut counter: i32 = 0;
    let mut previous: i32;
    let mut current: i32;
    for n in 0..=numbers.len() - 2 {
        previous = numbers[n];
        current = numbers[n + 1];
        if current > previous {
            counter = counter + 1;
        }
    }

    return Ok(counter);
}

fn puzzle2(numbers: &Vec<i32>) -> io::Result<i32> {
    let mut counter: i32 = 0;
    let mut previous: i32;
    let mut current: i32;

    for n in 0..=numbers.len() - 4 {
        previous = numbers[n] + numbers[n + 1] + numbers[n + 2];
        current = numbers[n + 1] + numbers[n + 2] + numbers[n + 3];
        if current > previous {
            counter = counter + 1;
        }
    }

    return Ok(counter);
}

fn main() -> io::Result<()>
{
    let file = File::open("./input.txt")?;
    let buf_reader = io::BufReader::new(file);

    let mut numbers = Vec::new();
    for line in buf_reader.lines() {
        match line {
            Ok(v) => {
                numbers.push(v.parse::<i32>().unwrap());
            },
            Err(e) => println!("{}", e),
        }
    }

    let puzzle1_result = puzzle1(&numbers);
    let puzzle2_result = puzzle2(&numbers);

    match puzzle1_result {
        Ok(v) => println!("Puzzle 1: {}", v),
        Err(e) => println!("{}", e),
    }

    match puzzle2_result {
        Ok(v) => println!("Puzzle 2: {}", v),
        Err(e) => println!("{}", e),
    }

    Ok(())
}