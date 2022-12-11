use std::{
    cell::Cell,
    fs::File,
    io::{BufRead, BufReader},
};

const LIT: char = '█';
const DARK: char = '░';
const ROW_PITCH: i32 = 40;

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut clock = 0;
    let reg_x = Cell::new(1);

    let mut tick = || {
        let pos_x: i32 = clock % ROW_PITCH;
        if pos_x == 0 {
            println!("");
        }

        clock += 1;

        let pixel_lit = pos_x.abs_diff(reg_x.get()) <= 1;
        if pixel_lit {
            print!("{}", LIT);
        } else {
            print!("{}", DARK);
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
}
