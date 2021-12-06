use std::fs::File;
use std::io::{ BufRead, BufReader };

fn calculate_laternfish_growth(mut laternfish: [i64; 9], days: i32) -> i64 {
    for _i in 1..=days {
        let laternfish_at_zero_days = laternfish[0];
        for i in 0..=7 {
            laternfish[i] = laternfish[i + 1];
        }
        laternfish[8] = laternfish_at_zero_days;
        laternfish[6] += laternfish_at_zero_days;
    }

    let mut sum_laternfish = 0;
    for i in 0..=8 {
        sum_laternfish += laternfish[i];
    }

    return sum_laternfish;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut line: String = String::from(""); 
    let _result = buf_reader.read_line(&mut line);

    let mut laternfish: [i64; 9] = [0; 9];
    for number in line.split(",") {
        laternfish[number.parse::<i64>().unwrap() as usize] += 1;
    }

    println!("Puzzle 1: {}", calculate_laternfish_growth(laternfish.clone(), 80));
    println!("Puzzle 2: {}", calculate_laternfish_growth(laternfish.clone(), 256));
}