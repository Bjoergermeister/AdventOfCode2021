use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader };

fn puzzle1(polymer: &mut Vec<char>, rules: &HashMap<String, char>, char_count: &mut HashMap<char, i32>) -> i32 {
    for _step in 0..10 {
        let mut index = 0;
        while index < polymer.len() - 1 {
            let pair = format!("{}{}", polymer[index], polymer[index + 1]);        
            if rules.contains_key(&pair) {
                let new_character = rules[&pair];
                polymer.insert((index + 1) as usize, new_character);
                if char_count.contains_key(&new_character){
                    char_count.insert(new_character, char_count[&new_character] + 1);    
                }else{
                    char_count.insert(new_character, 1);
                }
            }
            index += 2;
        }
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
    let mut char_count = HashMap::<char, i32>::new();
    let mut polymer = Vec::<char>::new();
    for character in lines[0].as_ref().unwrap().chars() {
        if char_count.contains_key(&character) {
            char_count.insert(character, char_count[&character] + 1);
        }else{
            char_count.insert(character, 1);
        }
        polymer.push(character);
    }

    //Process rules
    let mut rules = HashMap::<String, char>::new();
    for i in 2..lines.len(){
        let line = lines[i].as_ref().unwrap();
        rules.insert(line[..2].to_string(), line.chars().nth(line.len() - 1).unwrap());
    }

    println!("Puzzle 1: {}", puzzle1(&mut polymer, &rules, &mut char_count));
}