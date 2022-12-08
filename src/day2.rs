use std::fs::File;
use std::io::{self, BufRead};

// If i won 6 points
// If draw 3 points
// Rock 1 point
// Paper 2 points
// Scissors 3 points

pub fn main() {
    let file = File::open("./src/day2.input").expect("Input file is not found");
    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    for line in lines {
        let line = line.expect("Couldn't recognize the line");

        let opp_choice = get_result(line.chars().nth(0).unwrap());
        let my_choice = get_result(line.chars().nth(2).unwrap());

        if my_choice == "rock" {
            match opp_choice.as_str() {
                "scissors" => total_score += 6,
                "rock" => total_score += 3,
                _ => (),
            }
            total_score += 1;
        } else if my_choice == "paper" {
            match opp_choice.as_str() {
                "rock" => total_score += 6,
                "paper" => total_score += 3,
                _ => (),
            }
            total_score += 2;
        } else if my_choice == "scissors" {
            match opp_choice.as_str() {
                "paper" => total_score += 6,
                "scissors" => total_score += 3,
                _ => (),
            }
            total_score += 3;
        }
    }

    println!("Total Score: {}", total_score);
}

fn get_result(val: char) -> String {
    match val {
        'A' | 'X' => "rock".to_owned(),
        'B' | 'Y' => "paper".to_owned(),
        'C' | 'Z' => "scissors".to_owned(),
        _ => panic!("Not an option"),
    }
    // if(val == "A" || val == "X") {
    //     return Op
    // }
}
