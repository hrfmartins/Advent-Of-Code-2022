use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total_sum = 0;
    if let Ok(all) = read_lines("input/input.txt") {
        for line in all {
            if let Ok(line) = line {
                let first_part = &line[..line.len() / 2];
                let second_part = &line[line.len() / 2..];
                let mut returning = HashSet::new();

                find_common(first_part, second_part, &mut returning);

                total_sum += calculate_sum(&returning)
            }
        }
    } else {
        println!("Problem reading")
    }

    println!("Final result: {}", total_sum);
}

fn find_common(first: &str, second: &str, vector: &mut HashSet<char>) {
    let set: HashSet<char> = second.chars().collect();

    for index in 0..first.len() {
        if set.contains(&first.chars().nth(index).unwrap()) {
            vector.insert(first.chars().nth(index).unwrap());
        }
    }
}

fn calculate_sum(vector: &HashSet<char>) -> u32 {
    let mut sum: u32 = 0;

    for element in vector {
        sum = sum + if element.is_uppercase() {
            *element as u32 - 38
        } else {
            *element as u32 - 96
        };
    }

    return sum;
}
