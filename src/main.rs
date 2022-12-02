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
            "A X" => 0 + 3, // i lose, scissors
            "A Y" => 3 + 1, // i draw, rock
            "A Z" => 6 + 2, // i win,  paper
            // picks paper
            "B X" => 0 + 1, // lose, rock
            "B Y" => 3 + 2, // draw, paper
            "B Z" => 6 + 3, // win, scissors
            // picks scissors
            "C X" => 0 + 2, // lose paper
            "C Y" => 3 + 3, // draw scissors
            "C Z" => 6 + 1, // win rock
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
