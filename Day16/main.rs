use std::fs::File;
use std::io::{BufRead, BufReader};

struct Packet {
    version: i32,
    type_id: i32,
    subpackets: Vec<Packet>,
    value: i128,
}

impl Packet {
    fn new() -> Packet {
        Packet {
            version: 0,
            type_id: 0,
            subpackets: Vec::<Packet>::new(),
            value: 0,
        }
    }

    fn calculate(&mut self) {
        if self.type_id == 4 {
            return;
        }

        let mut values = Vec::<i128>::new();
        for index in 0..self.subpackets.len() {
            let subpacket = &mut self.subpackets[index];
            subpacket.calculate();
            values.push(subpacket.value);
        }

        match self.type_id {
            0 => self.value = values.iter().sum(),
            1 => self.value = values.iter().fold(1i128, |sum, val| sum * val),
            2 => self.value = *values.iter().min().unwrap(),
            3 => self.value = *values.iter().max().unwrap(),
            5 => self.value = (values[0] > values[1]) as i128,
            6 => self.value = (values[0] < values[1]) as i128,
            7 => self.value = (values[0] == values[1]) as i128,
            _ => println!("Something else"),
        }
    }
}

fn to_binary(character: &char) -> i32 {
    let ascii_code = *character as i32;
    let code = match ascii_code {
        48..=57 => ascii_code - 48,
        65..=70 => ascii_code - 55,
        _ => 0,
    };

    return code;
}

fn extract_value_from_message(message: &Vec<i32>, start: usize, end: usize) -> i128 {
    let mut number: u128 = 0;
    for index in start..=end {
        let power = (end - index) as u32;
        number += (message[index] as i128 * 2_i128.pow(power)) as u128;
    }

    return number as i128;
}

fn parse_packet(message: &Vec<i32>, mut index: usize) -> (Packet, usize) {
    let mut packet = Packet::new();

    packet.version = extract_value_from_message(&message, index, index + 2) as i32;
    packet.type_id = extract_value_from_message(&message, index + 3, index + 5) as i32;

    let length: usize;
    if packet.type_id == 4 {
        index += 6;
        let mut bits = Vec::<i32>::new();

        loop {
            bits.extend_from_slice(&message[index + 1..=index + 4]);
            index += 5;
            if message[(index - 5) as usize] == 0 {
                break;
            }
        }

        packet.value = extract_value_from_message(&bits, 0, bits.len() - 1);
    } else {
        let length_type = extract_value_from_message(&message, index + 6, index + 6);
        index += 7;

        let subpacket_length;
        if length_type == 0 {
            length = extract_value_from_message(&message, index, index + 14) as usize;
            index += 15;
            subpacket_length = index + length;
        } else {
            length = extract_value_from_message(&message, index, index + 10) as usize;
            index += 11;
            subpacket_length = length;
        }

        let mut current_subpacket = if length_type == 0 { index } else { 0 };
        let mut subpacket_index = index;
        while current_subpacket < subpacket_length {
            let (subpacket, new_subpacket_index) = parse_packet(&message, subpacket_index);
            packet.subpackets.push(subpacket);
            subpacket_index = new_subpacket_index;
            current_subpacket = if length_type == 0 {
                new_subpacket_index
            } else {
                current_subpacket + 1
            };
        }
        index = subpacket_index;
    }

    (packet, index)
}

fn sum_versions(packets: &Vec<Packet>) -> i32 {
    let mut sum = 0;
    for packet in packets {
        sum += packet.version;
        sum += sum_versions(&packet.subpackets);
    }

    return sum;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    reader.read_line(&mut line);

    let chars = line.chars();
    let mut message = Vec::<i32>::new();
    for character in chars {
        let mut character_in_binary = vec![0; 4];
        let mut number = to_binary(&character);
        for i in 0..4 {
            character_in_binary[3 - i] = number % 2;
            number /= 2;
        }
        message.append(&mut character_in_binary);
    }

    let (mut packet, _index) = parse_packet(&message, 0);

    let sum = packet.version + sum_versions(&packet.subpackets);

    packet.calculate();

    println!("Puzzle 1: {}", sum);
    println!("Puzzle 2: {}", packet.value);
}
