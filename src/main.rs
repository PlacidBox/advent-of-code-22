use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input-day-2.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut score = 0;

    for line in lines {
        let line = line.unwrap();
        score += match line.as_str() {
            // opponent picks rock
            "A X" => 1 + 3, // i pick rock
            "A Y" => 2 + 6, // paper
            "A Z" => 3,     // scissors
            // picks paper
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            // picks scissors
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            unknown => {
                println!("unexpected entry {}", unknown);
                0
            }
        };
    }

    println!("total score: {}", score);
}

fn _day1() {
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
