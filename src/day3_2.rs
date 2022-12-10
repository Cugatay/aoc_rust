use std::fs::File;
use std::io::{self, BufRead};

// If i won 6 points
// If draw 3 points
// Rock 1 point
// Paper 2 points
// Scissors 3 points

pub fn main() {
    let file = File::open("./src/day3.input").expect("Input file is not found");
    let mut lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let line1 = line1.expect("Couldn't read the line");
        let line2 = line2.expect("Couldn't read the line");
        let line3 = line3.expect("Couldn't read the line");

        let set1 = line1.chars();
        let set2 = line2.chars();
        let set3 = line3.chars();

        let mut common_char: Option<char> = None;

        for char1 in set1 {
            for char3 in set3.to_owned() {
                if common_char != None {
                    break;
                }

                let exist = set2
                    .clone()
                    .find(|char2| char2.to_owned() == char3 && char3 == char1);

                match exist {
                    Option::None => {
                        // break;
                    }
                    _ => {
                        common_char = exist;
                    }
                }
            }
        }
        let common_char = common_char.unwrap();

        total_score += common_char.to_digit(36).unwrap() - 9;
        if common_char.is_uppercase() {
            total_score += 26
        }
    }

    println!("Total: {}", total_score);
}
