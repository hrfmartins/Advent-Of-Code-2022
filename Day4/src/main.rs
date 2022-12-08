use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut total_overlap = 0;
        let mut total_overlap_2 = 0;
        // Consumes the iterator, returns an (Optional) String
        let mut ranges = vec![] as Vec<(Vec<i32>, Vec<i32>)>;
        for line in lines {
            if let Ok(ip) = line {
                let string_split: Vec<&str>= ip.split(",").collect();
                let range1: Vec<i32>= string_split[0].split("-").map(|x: &str| x.parse().unwrap()).collect();
                let range2: Vec<i32>= string_split[1].split("-").map(|x: &str| x.parse().unwrap()).collect();

                ranges.insert(0, (range1, range2));                
            }
        }

        for i in 0..ranges.len() {
            let (range1, range2) = &ranges[i];
            if range1[0] <= range2[0] && range1[1] >= range2[1] {
                total_overlap += 1
            }
            else if range2[0] <= range1[0] && range2[1] >= range1[1] {
                total_overlap += 1
            }


            if cmp::max(range1[0], range2[0]) - cmp::min(range1[1], range2[1]) <=0 {
                total_overlap_2 += 1
            }
            
        }

        println!("1: {}", total_overlap);
        println!("2: {}", total_overlap_2);
    }
}