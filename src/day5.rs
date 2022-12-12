use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader};

pub fn main() {
    let file = File::open("./src/day5.input").expect("Input file is not found");

    let mut lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    lines[..8].reverse();

    for line in &lines[..8] {
        if !line.starts_with("move") {
            for (index, chr) in line.chars().enumerate() {
                if chr.is_uppercase() {
                    let stack = (index - 1) / 4;
                    stacks[stack].push(chr);
                }
            }
        } else {
        }
    }

    for line in &lines[10..] {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let quantity = parts[1].parse::<i32>().unwrap();
        let from = parts[3].parse::<i32>().unwrap() - 1;
        let to = parts[5].parse::<i32>().unwrap() - 1;

        let minus = stacks[from as usize].len() - (quantity as usize);
        stacks[from as usize][minus..].reverse();

        for i in 0..quantity {
            let from_stack = &stacks.to_owned()[from as usize];

            let move_value = from_stack[from_stack.len() - 1];
            stacks[to as usize].push(move_value);
            stacks[from as usize].remove(from_stack.len() - 1);
        }
    }

    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
}
