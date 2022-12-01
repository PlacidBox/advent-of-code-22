use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input-day-1.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut this_elf = 0;

    let mut elves = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(this_elf);
            this_elf = 0;
        } else {
            let cal: i32 = line.parse().unwrap();
            this_elf += cal;
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!("calories {}, {}, {}", elves[0], elves[1], elves[2]);
}
