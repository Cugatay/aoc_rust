use std::fs::File;
use std::io::{self, BufRead};

// If i won 6 points
// If draw 3 points
// Rock 1 point
// Paper 2 points
// Scissors 3 points

pub fn main() {
    let file = File::open("./src/day3.input").expect("Input file is not found");
    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    for line in lines {
        let line = line.expect("Couldn't recognize the line");
        let (a, b) = line.split_at(line.len() / 2);
        let a_charset = a.chars();
        let b_charset = b.chars();

        let mut common_chars: Vec<char> = Vec::new();

        for letter_a in a_charset {
            for letter_b in b_charset.to_owned() {
                if letter_b == letter_a {
                    let is_exists = common_chars.iter().find(|&c| letter_b == c.to_owned());

                    match is_exists {
                        Option::None => {
                            common_chars.push(letter_b);
                            total_score += letter_b.to_digit(36).unwrap() - 9;
                            if (letter_b.is_uppercase()) {
                                total_score += 26
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    println!("{:?}", total_score);
}
