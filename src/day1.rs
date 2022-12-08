use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("./src/day1.input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut elves: Vec<i32> = Vec::new();

    let mut temp_val: i32 = 0;

    for line in lines {
        let line = line.expect("Couldn't recognize the line");

        if line == "" {
            elves.push(temp_val);
            temp_val = 0;
        } else {
            let val = line
                .parse::<i32>()
                .expect("The value of the line can't be parsed to i32");
            temp_val += val;
        }
    }
    let result = elves.iter().max().unwrap();
    println!("Result: {:?}", result);
}
