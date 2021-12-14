use std::fs::File;
use std::io::{ BufRead, BufReader };

fn puzzle1(lines: &Vec<String>) -> i32 {
    let mut number_count: i32 = 0;
    let mut matching_numbers = vec![2, 3, 4, 7];
    for line in lines {
        let parts: Vec<&str> = line.split("|").collect();
        for number in parts[1].split_whitespace() {            
            if matching_numbers.contains(&(number.len() as i32)) {
                number_count = number_count + 1;
            }
        }
    }

    return number_count;
}

fn main(){
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    for line in buf_reader.lines(){
        lines.push(line.unwrap())
    }

    println!("Puzzle 1: {}", puzzle1(&lines));
}