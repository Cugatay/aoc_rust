use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

pub fn main() {
    let mut count = 0;

    let file = File::open("./src/day4.input").expect("Input file is not found");
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let line = line.expect("Couldn't recognize the line");

        let ranges = line.split(",");

        let mut rg: Vec<Range<i32>> = Vec::new();

        ranges.for_each(|range| {
            let mut limits = range.split("-");
            let (limit1, limit2) = (
                limits.next().unwrap().parse::<i32>().unwrap(),
                limits.next().unwrap().parse::<i32>().unwrap(),
            );

            rg.push(limit1..limit2);
        });

        println!("Inside: {:?}", rg);
        if does_contain(&rg[0], &rg[1]) {
            // println!("This is it {}", line);
            count += 1;
        }
    }

    println!("Count: {}", count)
}

fn does_contain(range1: &Range<i32>, range2: &Range<i32>) -> bool {
    println!("ranges: {:?} {:?}", range1, range2);

    let start1 = range1.start;
    let end1 = range1.end;
    let start2 = range2.start;
    let end2 = range2.end;

    // if range1.contains(&range2.start) {
    //     println!("this does");
    //     return true;
    // }
    if end1 >= start2 && end2 >= start1 {
        println!("this does");
        return true;
    }

    // if range1.start <= range2.start && range1.end >= range2.end {
    //     return true;
    // }
    //
    // if range2.start <= range1.start && range2.end >= range1.end {
    //     return true;
    // }

    return false;
}
