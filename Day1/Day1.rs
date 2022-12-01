use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    let mut elf: i32 = 0;
    let mut max_load: i32 = 0;
    let mut elf_carry_list: Vec<i32> = vec![0];

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    elf+=1;
                    elf_carry_list.push(0);
                } else {
                    elf_carry_list[elf as usize] = elf_carry_list[elf as usize] + ip.parse::<i32>().unwrap();
                    
                    if max_load < elf_carry_list[elf as usize] {
                        max_load = elf_carry_list[elf as usize]
                    }
                }
            }
        }
    }

    print!("Max load: {}\n", max_load);

    let mut sum_top: i32 = 0;
    elf_carry_list.sort_by(|x, y| y.cmp(&x));

    for elf in 0..3 {
        sum_top += elf_carry_list[elf as usize]
    }

    print!("Sum top 3: {}", sum_top)
}