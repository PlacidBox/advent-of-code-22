use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FIELD_SIZE: usize = 99;

// in [row][column] order. shouldn't matter for this puzzles, anyway
fn load_input() -> [[u8; FIELD_SIZE]; FIELD_SIZE] {
    let mut res = [[0; FIELD_SIZE]; FIELD_SIZE];
    let mut this_row = 0;

    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    for line in lines {
        let row = &mut res[this_row];

        let mut this_char = 0;
        for ch in line.unwrap().bytes() {
            let v = ch - b'0';
            row[this_char] = v;

            assert!(v < 10);
            this_char += 1;
        }

        this_row += 1;
    }

    res
}

fn main() {
    let input = load_input();

    let mut num_visible = 0;

    for y in 0..FIELD_SIZE {
        for x in 0..FIELD_SIZE {
            if is_visible(&input, x, y) {
                num_visible += 1;
            }
        }
    }

    println!("Trees visible: {}", num_visible);
}

fn is_visible(input: &[[u8; FIELD_SIZE]; FIELD_SIZE], x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == FIELD_SIZE - 1 || y == FIELD_SIZE - 1 {
        return true;
    }

    let this_height = input[y][x];

    // check to left. if all trees to the left are lower, than visible to the left
    let vis_left = (0..x).all(|check_x| input[y][check_x] < this_height);
    let vis_right = (x + 1..FIELD_SIZE).all(|check_x| input[y][check_x] < this_height);

    let vis_up = (0..y).all(|check_y| input[check_y][x] < this_height);
    let vis_down = (y + 1..FIELD_SIZE).all(|check_y| input[check_y][x] < this_height);

    vis_left || vis_right || vis_up || vis_down
}
