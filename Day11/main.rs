use std::fs::File;
use std::io::{ BufRead, BufReader };

use std::collections::VecDeque;

fn increase_surrounding_squids(grid: &mut Vec<i32>, position: i32, squids_to_flash: &mut VecDeque<usize>, flashed_squids: &mut Vec<usize>){
    let x: i32 = position % 10;
    let y: i32 = position / 10;
    let range = std::ops::Range::<i32> { start: -1, end: 2 };
    for dx in range.clone() {
        for dy in range.clone() {
            if dx == 0 && dy == 0 {
                continue;
            }

            let xx: i32 = (x + dx) as i32;
            let yy: i32 = (y + dy) as i32;
            if xx < 0 || xx > 9 {
                continue;
            }
            if yy < 0 || yy > 9 {
                continue;
            }
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_position = (yy * 10 + xx) as usize;
            grid[new_position] += 1;
            if grid[new_position] > 9 && flashed_squids.contains(&new_position) == false {
                squids_to_flash.push_back(new_position);
            }
        }
    }
}

fn puzzle1(grid: &mut Vec<i32>) -> i32 {
    let mut flashes = 0;
    let mut flashed_squids = Vec::<usize>::new();
    let mut squids_to_flash = VecDeque::<usize>::new();
    for _step in 1..=100 {
        //Increase energy level
        for i in 0..grid.len() {
            grid[i] += 1;
            if grid[i] == 10 {
                squids_to_flash.push_back(i);
            }
        }
        
        //Let squids flash
        while squids_to_flash.len() > 0 {
            let squid_to_flash = squids_to_flash.pop_front().unwrap();
            if flashed_squids.contains(&squid_to_flash) == false {
                increase_surrounding_squids(grid, squid_to_flash as i32, &mut squids_to_flash, &mut flashed_squids);
                flashed_squids.push(squid_to_flash);
                flashes += 1;
            }
        }
        
        //Reset all flashed squids
        for i in 0..flashed_squids.len() {
            grid[flashed_squids[i]] = 0;
        }
        squids_to_flash.clear();
        flashed_squids.clear();        
    }
    
    flashes
}

fn main(){
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut grid = vec![100; 0];
    for line_result in buf_reader.lines() {                
        let line = line_result.unwrap();
        let line_bytes = line.as_bytes();
        for i in 0..line.len() {
            grid.push(line_bytes[i] as i32 - 0x30); //Convert char to i32
        }
    }

    println!("Puzzle 1: {}", puzzle1(&mut grid));
}