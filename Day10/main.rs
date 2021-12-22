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
        '}' => return 1197,
        ']' => return 57,
        '>' => return 25137,
        _ => return 0, 
    }
}

fn is_open_bracket(bracket: char) -> bool {
    return bracket == '(' || bracket == '{' || bracket == '[' || bracket == '<';
}

fn puzzle1(lines: &Vec<String>) -> i32 
{
    let mut stack: Vec<char> = Vec::new();
    let mut score = 0; 
    for i in 0..lines.len(){
        let line_chars: Vec<char> = lines[i].chars().collect();
        for j in 0..line_chars.len() {
            let bracket: char = line_chars[j];
            if is_open_bracket(bracket) {
                stack.push(bracket);
            }else{
                let last_open_bracket = stack.pop().unwrap();
                if get_inverse_for_bracket(last_open_bracket) != bracket {
                    score += get_score_for_bracket(bracket);
                    break;
                }
            }
        }
    }
    return score;
}

fn main()
{
    let file = File::open("input.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in buf_reader.lines(){
        lines.push(line.unwrap());
    }

    println!("Puzzle 1: {}", puzzle1(&lines));
}