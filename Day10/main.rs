use std::fs::File;
use std::io::{ BufRead, BufReader };

fn get_inverse_for_bracket(bracket: char) -> char {
    match bracket {
        '(' => return ')',
        '{' => return '}',
        '[' => return ']',
        '<' => return '>',
        _ => return ' '
    }
}

fn get_score_for_bracket(bracket: char) -> i32 {
    match bracket {
        ')' => return 3,
        ']' => return 57,
        '}' => return 1197,
        '>' => return 25137,
        _ => return 0, 
    }
}

fn is_open_bracket(bracket: char) -> bool {
    return bracket == '(' || bracket == '{' || bracket == '[' || bracket == '<';
}

fn puzzle1(lines: &Vec<String>) -> (i32, i64) 
{
    let mut corrupted_score = 0;
    let mut incomplete_scores = Vec::new(); 
    for i in 0..lines.len(){
        let mut stack: Vec<char> = Vec::new();
        let line_chars: Vec<char> = lines[i].chars().collect();
        let mut line_is_corrupted: bool = false;
        for j in 0..line_chars.len() {
            let bracket = line_chars[j];
            if is_open_bracket(bracket) {
                stack.push(bracket);
            }else{
                let last_open_bracket = stack.pop().unwrap();
                if get_inverse_for_bracket(last_open_bracket) != bracket {
                    corrupted_score += get_score_for_bracket(bracket);
                    line_is_corrupted = true;
                    break;
                }
            }
        }

        if line_is_corrupted == false {
            let mut incomplete_score: i64 = 0;
            for i in 0..stack.len() {
                match get_inverse_for_bracket(stack[stack.len() - i - 1]){
                    ')' => incomplete_score = incomplete_score * 5 + 1,
                    ']' => incomplete_score = incomplete_score * 5 + 2,
                    '}' => incomplete_score = incomplete_score * 5 + 3,
                    '>' => incomplete_score = incomplete_score * 5 + 4,
                    _ => incomplete_score += 0,
                }
            }
            incomplete_scores.push(incomplete_score);
        }
    }

    incomplete_scores.sort();
    return (corrupted_score, incomplete_scores[(incomplete_scores.len() -1) / 2]);
}

fn main()
{
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in buf_reader.lines(){
        lines.push(line.unwrap());
    }

    let (puzzle1, puzzle2) = puzzle1(&lines);
    println!("Puzzle 1: {}", puzzle1);
    println!("Puzzle 2: {}", puzzle2);
}