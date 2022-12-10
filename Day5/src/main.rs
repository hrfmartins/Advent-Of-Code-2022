use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut init_state = vec![
        vec!["C", "Z", "N", "B", "M", "W", "Q", "V"],
        vec!["H", "Z", "R", "W", "C", "B"],
        vec!["F", "Q", "R", "J"],
        vec!["Z", "S", "W", "H", "F", "N", "M", "T"],
        vec!["G", "F", "W", "L", "N", "Q", "P"],
        vec!["L", "P", "W"],
        vec!["V", "B", "D", "R", "G", "C", "Q", "J"],
        vec!["Z", "Q", "N", "B", "W"],
        vec!["H", "L", "F", "C", "G", "T", "J"],
    ];

    let mut init_state_2 = init_state.clone();

    if let Ok(lines) = read_lines("./input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (num_moves, source, destination) =
                    sscanf!(ip, "move {i32} from {i32} to {i32}").unwrap();
                    for i in 0..num_moves {
                        let popped = init_state[(source - 1) as usize].pop().unwrap();
                        init_state[(destination - 1) as usize].push(popped);
                    }
            }
        }
    }
    
    print!("Part 1: ");
    for i in 0..init_state.len() {
        print!("{}", init_state[i][init_state[i].len() - 1])
    }
    println!("\n");


    // Part 2
    if let Ok(lines) = read_lines("./input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (num_moves, source, destination) =
                    sscanf!(ip, "move {i32} from {i32} to {i32}").unwrap();

                    let mut aux = vec![];
                    for i in 0..num_moves {
                        aux.insert(0, init_state_2[(source - 1) as usize].pop().unwrap());
                    }
                    init_state_2[(destination - 1) as usize].append(&mut aux);
            }
        }
    }

    print!("Part 2: ");
    for i in 0..init_state_2.len() {
        print!("{}", init_state_2[i][init_state_2[i].len() - 1])
    }
    println!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
