use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::cmp;

mod line;
use line::*;

fn update_board(board: &mut Vec<i32>, board_width: i32, line: &Line, use_diagonal: bool) -> i32 {
    let mut position_where_at_least_two_lines_overlap = 0;
    if line.is_vertical(){
        let y1 = cmp::min(line.y1, line.y2);
        let y2 = cmp::max(line.y1, line.y2);
        for i in y1..=y2 {
            let index = (i - 1) * board_width + (line.x1 - 1);
            board[index as usize] += 1;
            if board[index as usize] == 2 {
                position_where_at_least_two_lines_overlap += 1;
            }
        }
    }

    if line.is_horizontal(){
        let x1 = cmp::min(line.x1, line.x2);
        let x2 = cmp::max(line.x1, line.x2);
        let first_index = (line.y1 - 1) * board_width + x1;
        for i in 0..=(x2 - x1) {
            board[(first_index + (i - 1)) as usize] += 1;
            if board[(first_index + (i - 1)) as usize] == 2 {
                position_where_at_least_two_lines_overlap += 1;
            }
        }
    }

    if line.is_diagonal() && use_diagonal {
        let x1 = cmp::min(line.x1, line.x2);
        let x2 = cmp::max(line.x1, line.x2);
        let y1 = cmp::min(line.y1, line.y2);
        let y2 = cmp::max(line.y1, line.y2);
    }

    return position_where_at_least_two_lines_overlap;
}

fn puzzle1(board_width: i32, board_height: i32, lines: &Vec<Line>) -> i32 {
    let mut board = vec![0; (board_width * board_height) as usize];
    let mut positions_where_at_least_two_lines_overlap = 0;
    for i in 0..lines.len() {
        positions_where_at_least_two_lines_overlap += update_board(&mut board, board_width, &lines[i], false);
    }

    return positions_where_at_least_two_lines_overlap;
}

fn puzzle2(board_width: i32, board_height: i32, lines: &Vec<Line>) -> i32 {
    let mut board = vec![0; (board_width * board_height) as usize];
    let mut positions_where_at_least_two_lines_overlap = 0;
    for i in 0..lines.len() {
        positions_where_at_least_two_lines_overlap += update_board(&mut board, board_width, &lines[i], false);
    }

    return positions_where_at_least_two_lines_overlap;
}

fn parse_point(point: &str) -> (i32, i32) {
    let coordinates = point.split(",").collect::<Vec<&str>>();
    let x = coordinates[0].parse::<i32>().unwrap();
    let y = coordinates[1].parse::<i32>().unwrap();
    return (x, y);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max_x = 0;
    let mut max_y = 0;

    let mut lines = Vec::new();
    for line_result in reader.lines() {

        let line_string = line_result.unwrap();
        let points = line_string.split(" -> ").collect::<Vec<&str>>();
        let (x1, y1) = parse_point(points[0]);
        let (x2, y2) = parse_point(points[1]);
        
        lines.push(Line::new(x1, y1, x2, y2));

        //Update max bounds
        max_x = cmp::max(max_x, cmp::max(x1, x2));
        max_y = cmp::max(max_y, cmp::max(y1, y2));
        
    }
    println!("{}, {}", max_x, max_y);
    println!("Puzzle 1: {}", puzzle1(max_x, max_y, &lines));
    println!("Puzzle 2: {}", puzzle2(max_x, max_y, &lines));
}