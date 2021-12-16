use std::fs::File;
use std::io::{ BufRead, BufReader };

fn puzzle1(grid: Vec<i32>, line_length: usize) -> i32 {
    let mut risk_level_sum = 0;
    for i in 0..grid.len() {
        let left = (i % line_length == 0) || (grid[i - 1] > grid[i]);
        let up = (i < line_length) || (grid[i - line_length] > grid[i]);
        let right = (i % line_length == line_length - 1) || (grid[i + 1] > grid[i]);
        let down = (i >= grid.len() - line_length) || (grid[i + line_length] > grid[i]);

        if left && up && right && down {
            risk_level_sum += grid[i] + 1;
        }
    }
    return risk_level_sum;
}

fn main(){
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut grid_size: usize = 0;
    let mut grid = Vec::new();
    for line_result in buf_reader.lines() {                
        let line = line_result.unwrap();
        let line_bytes = line.as_bytes();
        for i in 0..line.len() {
            grid.push(line_bytes[i] as i32 - 0x30); //Convert char to i32
        }        
        grid_size = line.len();
    }

    println!("Puzzle 1: {}", puzzle1(grid, grid_size));
}