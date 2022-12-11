use std::{
    cell::Cell,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut clock = 0;
    let reg_x = Cell::new(1);
    let mut signal_sum = 0;

    let mut tick = || {
        clock += 1;
        if [20, 60, 100, 140, 180, 220].contains(&clock) {
            let sig_strength = clock * reg_x.get();
            println!("{:>6} str {}", clock, sig_strength);
            signal_sum += sig_strength;
        }
    };

    for line in lines {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();

        match parts.as_slice() {
            ["noop"] => tick(),
            ["addx", amount] => {
                tick();
                tick();
                reg_x.set(reg_x.get() + amount.parse::<i32>().unwrap());
            }
            _ => panic!(),
        }
    }

    println!("total: {}", signal_sum);
}
