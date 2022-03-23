use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

fn puzzle1(steps: i32, mut pairs: HashMap<String, i64>, rules: &HashMap<String, char>, mut char_count: HashMap<char, i64>) -> i64 {
    for _step in 0..steps {
        let mut new_pairs = HashMap::<String, i64>::new();
        for pair in pairs.into_iter() {
            let first: char = pair.0.chars().nth(0).unwrap();
            let second: char = pair.0.chars().nth(1).unwrap();
            let insert: char = rules[&pair.0];

            let first_new_pair = format!("{}{}", first, insert);
            let second_new_pair = format!("{}{}", insert, second);

            *char_count.entry(insert).or_insert(0) += pair.1;
            
            *new_pairs.entry(first_new_pair).or_insert(0) += pair.1;
            *new_pairs.entry(second_new_pair).or_insert(0) += pair.1;
        }
        pairs = new_pairs.clone();
    }

    //Find largest and smallest character counts
    let mut min_count = -1;
    let mut max_count = 0;
    for count in char_count.values() {
        max_count = std::cmp::max(max_count, *count);
        if min_count == -1 || min_count > *count {
            min_count = *count;
        }
    }

    return max_count - min_count;
}

fn main(){
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<std::io::Result<String>> = reader.lines().collect();

    //Process first line
    let mut char_count = HashMap::<char, i64>::new();
    let mut initial_pairs = HashMap::<String, i64>::new();

    let starting_polymer = lines[0].as_ref().unwrap();
    for i in 0..starting_polymer.len() - 1 {
        let pair = starting_polymer[i..=(i + 1)].to_string();
        *char_count.entry(pair.chars().nth(1).unwrap()).or_insert(0) += 1;
        *initial_pairs.entry(pair).or_insert(0) += 1;
    } 
    *char_count.entry(starting_polymer.chars().nth(0).unwrap()).or_insert(0) += 1;

    //Process rules
    let mut rules = HashMap::<String, char>::new();
    for i in 2..lines.len(){
        let line = lines[i].as_ref().unwrap();
        rules.insert(line[..2].to_string(), line.chars().nth(line.len() - 1).unwrap());
    }

    println!("Puzzle 1: {}", puzzle1(10, initial_pairs.clone(), &rules, char_count.clone()));
    println!("Puzzle 2: {}", puzzle1(40, initial_pairs.clone(), &rules, char_count.clone()));
}