use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input-day-1.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut this_elf = 0;
    let mut most_calories = 0;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            most_calories = most_calories.max(this_elf);
            this_elf = 0;
        } else {
            let cal: i32 = line.parse().unwrap();
            this_elf += cal;
        }
    }

    println!("Most calories: {}", most_calories);
}
