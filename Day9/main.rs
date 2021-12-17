use std::fs::File;
use std::io::{ BufRead, BufReader };

fn puzzle1(grid: &Vec<i32>, line_length: usize, low_points: &mut Vec<usize>) -> i32 {
    let mut risk_level_sum = 0;
    for i in 0..grid.len() {
        let left = (i % line_length == 0) || (grid[i - 1] > grid[i]);
        let up = (i < line_length) || (grid[i - line_length] > grid[i]);
        let right = (i % line_length == line_length - 1) || (grid[i + 1] > grid[i]);
        let down = (i >= grid.len() - line_length) || (grid[i + line_length] > grid[i]);

        if left && up && right && down {
            low_points.push(i);
            risk_level_sum += grid[i] + 1;
        }
    }
    return risk_level_sum;
}

fn calculate_basin_size(grid: &Vec<i32>, line_length: usize, current_position: usize, current_basin: &mut Vec<usize>) -> i32 {
    current_basin.push(current_position);
    let mut basin_size = 1;

    if current_position % line_length != 0 && !current_basin.contains(&(current_position - 1)) {
        if grid[current_position] < grid[current_position - 1] && grid[current_position - 1] < 9 {
            basin_size += calculate_basin_size(grid, line_length, current_position - 1, current_basin);
        }
    }

    if current_position >= line_length && !current_basin.contains(&(current_position - line_length)) {
        if grid[current_position] < grid[current_position - line_length] && grid[current_position - line_length] < 9 {
            basin_size += calculate_basin_size(grid, line_length, current_position - line_length, current_basin);
        }
    }

    if current_position % line_length != line_length - 1 && !current_basin.contains(&(current_position + 1)){
        if grid[current_position] < grid[current_position + 1] && grid[current_position + 1] < 9 {
            basin_size += calculate_basin_size(grid, line_length, current_position + 1, current_basin);
        }
    }

    if current_position < grid.len() - line_length && !current_basin.contains(&(current_position + line_length)){
        if grid[current_position] < grid[current_position + line_length] && grid[current_position + line_length] < 9 {
            basin_size += calculate_basin_size(grid, line_length, current_position + line_length, current_basin);
        }
    }
    return basin_size;
}
fn puzzle2(grid: &Vec<i32>, line_length: usize, low_points: &Vec<usize>) -> i32 {
    let mut all_basins = Vec::new();
    let mut basin_sizes = Vec::new();
    for low_point in low_points {
        let mut current_basin = Vec::new();
        basin_sizes.push(calculate_basin_size(grid, line_length, *low_point, &mut current_basin));
        for i in 0..current_basin.len() {
            all_basins.push(current_basin[i]);
        }
    }

    basin_sizes.sort();

    let n = basin_sizes.len();
    return basin_sizes[n - 1] * basin_sizes[n - 2] * basin_sizes[n - 3];
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

    let mut low_points = Vec::new();
    println!("Puzzle 1: {}", puzzle1(&grid, grid_size, &mut low_points));
    println!("Puzzle 2: {}", puzzle2(&grid, grid_size, &low_points));
}