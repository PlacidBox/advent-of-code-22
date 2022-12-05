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

        let x_range = x_low..=x_high;
        let y_range = y_low..=y_high;

        if x_range.contains(&y_low) || x_range.contains(&y_high) {
            score += 1;
        } else if y_range.contains(&x_low) || y_range.contains(&x_high) {
            score += 1;
        }
    }

    println!("score {}", score);
}
