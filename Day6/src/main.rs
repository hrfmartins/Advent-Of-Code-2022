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
    let mut a = vec![];
    if let Ok(lines) = read_lines("./input/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                a.push(ip)
            }
        }
    }

    'outer: for word in &a {
        for i in (0..word.len()) {
            let mut hash_set = HashSet::new();

            hash_set.insert(word.chars().nth(i).unwrap());
            hash_set.insert(word.chars().nth(i + 1).unwrap());
            hash_set.insert(word.chars().nth(i + 2).unwrap());
            hash_set.insert(word.chars().nth(i + 3).unwrap());

            if  hash_set.len() == 4 {
                println!("FOUND IT! it's {}", i + 3 + 1);
                break 'outer
            }
        }
    }

    'outer: for word in &a {
        for i in 0..word.len() {
            let mut hash_set = HashSet::new();

            for j in 0..14 {
                hash_set.insert(word.chars().nth(i + j).unwrap());
            }
            
            if  hash_set.len() == 14 {
                println!("FOUND THE SECOND PART! it's {}", i + 13 + 1);
                break 'outer
            }
        }
    }
    
        
}
