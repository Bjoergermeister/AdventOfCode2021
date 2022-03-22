use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::cmp;

fn count_dots(paper: &Vec<i32>, width: i32, height: i32) -> i32 {
    let mut count: i32 = 0;
    for x in 0..width {
        for y in 0..height {
            if paper[(y * width + x) as usize] == 1 {
                count += 1;
            }
        }
    }

    count
}

fn fold(paper: &mut Vec<i32>, width: i32, height: i32, instructions: Vec<(char, i32)>) {
    for i in 0..instructions.len(){
        let axis: char = instructions[i].0;
        let number: i32 = instructions[i].1;
    
        if axis == 'x' {
            for x in 0..number {
                for y in 0..height {
                    let new_position = (y * width + x) as usize;
                    let old_position = (y * width + (number * 2 - x)) as usize;
                    if paper[old_position] == 1 {
                        paper[new_position] = paper[old_position];
                        paper[old_position] = 0;
                    }
                }
            }
        }
        if axis == 'y' {
            for x in 0..width {
                for y in 0..number {
                    let new_position = (y * width + x) as usize;
                    let old_position = ((number * 2 - y) * width + x) as usize;
                    if paper[old_position] == 1 {
                        paper[new_position] = paper[old_position];
                        paper[old_position] = 0;
                    }
                }
            }
        }
        if i == 0 {
            println!("Puzzle 1: {}", count_dots(paper, width, height))
        }
    }
}

fn main(){
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut min_x: i32 = -1;
    let mut max_x: i32 = -1;
    let mut min_y: i32 = -1;
    let mut max_y: i32 = -1;
    
    let mut points = Vec::<(i32, i32)>::new();
    let mut fold_instructions = Vec::<(char, i32)>::new();

    let mut is_point_line: bool = true;

    //Extract points
    for line_result in reader.lines(){
        let line = line_result.unwrap();
        
        if line.len() == 0 {
            is_point_line = false;
            continue;
        }

        if is_point_line {
            let parts: Vec<&str> = line.split(",").collect();
            let x = parts[0].parse::<i32>().unwrap();
            let y = parts[1].parse::<i32>().unwrap();

            //Find min and max coordinates
            max_x = cmp::max(max_x, x);
            max_y = cmp::max(max_y, y);
            if min_x == -1 || min_x > x {
                min_x = x;
            } 
            if min_y == -1 || min_y > y {
                min_y = y;
            }
            
            points.push((x, y))
        }else{
            let instruction: String = line.split(" ").collect::<Vec<&str>>()[2].to_string();
            let axis: char = instruction.chars().next().unwrap();
            let number: i32 = instruction[2..=instruction.len() - 1].parse::<i32>().unwrap();
            fold_instructions.push((axis, number));
        }
    }

    max_x += 1;
    max_y += 1;

    //Fill the "transparent paper"
    let mut paper = vec![0; (max_x * max_y) as usize];
    for point in points {
        let position = (point.1 * max_x + point.0) as usize;
        paper[position] = 1;
    }

    fold(&mut paper, max_x, max_y, fold_instructions);

    println!("Puzzle 2:");
    for y in 0..=5 {
        for x in 0..=38 {
            let position = (y * max_x + x) as usize;
            if paper[position] == 1 {
                print!("#");
            }else{
                print!(".")
            }
        }
        println!("");
    }
}