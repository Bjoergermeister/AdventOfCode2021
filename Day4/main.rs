use std::fs::File;
use std::io::{ BufRead, BufReader };

mod board;
use board::*;

fn puzzle1(numbers_to_draw: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in numbers_to_draw {
        for board in &mut *boards {
            board.remove_drawn_number(*number);
            if board.is_winner() {
                return board.get_score() * number;
            }
        }
    }
    return 0;
}

fn puzzle2(numbers_to_draw: &Vec<i32>, mut boards: Vec<Board>) -> i32 {
    for number in numbers_to_draw {        
        for i in (0..boards.len()).rev() { //Looping from back so objects can be removed from vector without skipping elements
            boards[i].remove_drawn_number(*number);
            if boards[i].is_winner() == false {
                continue;
            }
            
            if boards.len() == 1 {
                return boards[i].get_score() * number;
            }
            boards.remove(i);              
        }
    }
    return 0;
}

fn main(){
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut boards = Vec::new();
    let mut numbers_to_draw = Vec::new();
    let mut first_line = false;

    //Parse input
    for line_result in buf_reader.lines() {
        //First row contains all numbers which will be drawn, so first process these.
        if first_line == false {
            for number in line_result.unwrap().split(",") {
                numbers_to_draw.push(number.parse::<i32>().unwrap());
            }
            first_line = true;
            continue;
        }

        let line = line_result.unwrap();

        if line.len() == 0 {
            if lines.len() == 5 {
                boards.push(Board::new(&lines));
                lines.clear();
            }
        }else{
            lines.push(line);
        }
    }

    println!("Puzzle 1: {}", puzzle1(&numbers_to_draw, boards.clone()));
    println!("Puzzle 2: {}", puzzle2(&numbers_to_draw, boards.clone()));
} 