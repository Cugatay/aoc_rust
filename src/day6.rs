use std::fs;

pub fn main() {
    let marker_count = 14;

    let contents = fs::read_to_string("./src/day6.input").expect("File is not found");

    let mut charset = contents.chars().collect::<Vec<char>>();
    charset.pop();

    for (index, _) in charset.iter().enumerate() {
        let mut val: usize = 0;

        if index >= (marker_count - 1) {
            let mut set = charset[index - (marker_count - 1)..index + 1].to_owned();

            for chr in set.clone() {
                set.remove(0);

                let found = set.to_owned().into_iter().find(|c| c.to_owned() == chr);

                if found == Option::None {
                    val += 1;
                } else {
                    break;
                }
            }
        }

        if val == marker_count {
            println!("Answer: : {}", index + 1);
            break;
        }
    }
}
