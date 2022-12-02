use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    let mut points: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut chars = ip.chars();

                let enemy_input = Shape::match_input(chars.next().unwrap());
                chars.next();
                let my_input = Shape::match_input(chars.next().unwrap());

                points = points + my_input.get_score_for_shape();

                if my_input.get_winning() == enemy_input {
                    points = points + 6;
                } else if enemy_input == my_input {
                    points = points + 3;
                }
            }
        }

        println!("FINAL POINTS {}", points);
    }

}

#[derive(Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}


impl Shape {
    fn match_input(input: char) -> Self {
        match input {
            'A' | 'X' => return Shape::Rock,
            'B' | 'Y' => return Shape::Paper,
            'C' | 'Z' => return Shape::Scissors,
            _ => panic!("Invalid input")
        }    
    }

    fn get_score_for_shape(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn get_winning (&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

}


