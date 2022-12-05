use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input-day-4.txt").unwrap();
    let lines: std::io::Result<Vec<String>> = BufReader::new(input).lines().collect();

    let mut score = 0;

    for line in lines.unwrap() {
        let vals: Vec<&str> = line.split(&['-', ',']).collect();

        let x_low: i32 = vals[0].parse().unwrap();
        let x_high: i32 = vals[1].parse().unwrap();
        let y_low: i32 = vals[2].parse().unwrap();
        let y_high: i32 = vals[3].parse().unwrap();

        if x_low >= y_low && x_high <= y_high {
            // x contained within y
            score += 1;
        } else if y_low >= x_low && y_high <= x_high {
            // y contained within x
            score += 1;
        } else {
            println!("{}", line);
        }
    }

    println!("score {}", score);
}
