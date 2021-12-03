use std::fs::File;
use std::io::{BufRead, BufReader};

fn binary_to_decimal(binary: [i32; 12]) -> i32 {
    let mut decimal = 0;
    for i in 1..=binary.len() {
        let index: u32 = i as u32;
        decimal += binary[binary.len() - i] * 2_i32.pow(index - 1);
    }

    return decimal;
}

fn puzzle1(numbers: Vec<String>) -> i32 {
    let mut array: [i32; 12] = [0; 12];
    for number in numbers {
        for i in 0..number.len() {
            let value: char = number.chars().nth(i).unwrap();
            match value {
                '0' => array[i] -= 1,
                '1' => array[i] += 1,
                _ => (),
            };
        }        
    }

    let mut gamma_rate: [i32; 12] = [0; 12];
    let mut epsilon_rate: [i32; 12] = [1; 12];
    
    for i in 0..array.len() {
        if array[i] > 0 {
            gamma_rate[i] = 1;
            epsilon_rate[i] = 0;
        }
    }

    return binary_to_decimal(gamma_rate) * binary_to_decimal(epsilon_rate);
}

fn main()
{
    let file = File::open("input.txt").unwrap();
    let bufReader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in bufReader.lines() {
        lines.push(line.unwrap());
    }

    println!("Puzzle 1: {}", puzzle1(lines));
}