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
        let goal = get_goal(line.chars().nth(2).unwrap());

        let mut my_choice = "";

        if goal == "lose" {
            match opp_choice.as_str() {
                "rock" => my_choice = "scissors",
                "paper" => my_choice = "rock",
                "scissors" => my_choice = "paper",
                _ => (),
            }
        } else if goal == "draw" {
            match opp_choice.as_str() {
                "rock" => my_choice = "rock",
                "paper" => my_choice = "paper",
                "scissors" => my_choice = "scissors",
                _ => (),
            }
            total_score += 3;
        } else if goal == "win" {
            match opp_choice.as_str() {
                "rock" => my_choice = "paper",
                "paper" => my_choice = "scissors",
                "scissors" => my_choice = "rock",
                _ => (),
            }
            total_score += 6;
        }

        match my_choice {
            "rock" => total_score += 1,
            "paper" => total_score += 2,
            "scissors" => total_score += 3,
            _ => (),
        }
    }

    println!("Total Score: {}", total_score);
}

fn get_result(val: char) -> String {
    match val {
        'A' => "rock".to_owned(),
        'B' => "paper".to_owned(),
        'C' => "scissors".to_owned(),
        _ => panic!("Not an option"),
    }
}

fn get_goal(val: char) -> String {
    match val {
        'X' => "lose".to_owned(),
        'Y' => "draw".to_owned(),
        'Z' => "win".to_owned(),
        _ => panic!("Not an option"),
    }
}
