use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::{vec};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total_sum = 0;
    let mut current = 0;
    if let Ok(lines) = read_lines("input/input.txt") {    
        let mut group = vec![];    
        for line in lines {
            if let Ok(ip) = line {
                if current <= 2 {
                    group.insert(current, ip);
                }

                if current == 2 {
                    let mut common = HashSet::new();
                    find_common(&group[0], &group[1], &group[2], &mut common);

                    println!("{:?}", common);

                    total_sum += calculate_sum(&common);
                    current = 0;
                    continue;
                }
            
                
            }
            current += 1;
        }
    } else {
        println!("Problem reading")
    }

    println!("{}", total_sum);
}

fn find_common(first: &str, second: &str, third: &str, vector: &mut HashSet<char>) {
    let first_set: HashSet<char> = second.chars().collect();
    let third_set: HashSet<char> = third.chars().collect();

    for index in 0..first.len() {
        if first_set.contains(&first.chars().nth(index).unwrap()) && third_set.contains(&first.chars().nth(index).unwrap()) {
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
