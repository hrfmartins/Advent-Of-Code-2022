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
                let game_play = chars.next().unwrap();

                points = points + get_points_from_input(game_play) + enemy_input.turn_input_to_right_outcome(game_play).get_score_for_shape();
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
            'A' => return Shape::Rock,
            'B' => return Shape::Paper,
            'C' => return Shape::Scissors,
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

    fn get_defeat (&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn turn_input_to_right_outcome(self, input: char) -> Self {
        match input {
            'X' => return self.get_winning(),
            'Y' => return self,
            'Z' => return self.get_defeat(),
            _ => panic!("Invalid input")
        }
    }

}

fn get_points_from_input(input: char) -> i32 {
    match input {
        'X' => return 0,
        'Y' => return 3,
        'Z' => return 6,
        _ => panic!("Invalid input")
    }
}

