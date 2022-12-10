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

    let mut highest_score = 0;

    for y in 0..FIELD_SIZE {
        for x in 0..FIELD_SIZE {
            let score = scenic_score(&input, x, y);
            if score > highest_score {
                println!("most scenic at {} {} score {}", x, y, score);
                highest_score = score;
            }
        }
    }

    println!("best score: {}", highest_score);
}

fn scenic_score(input: &[[u8; FIELD_SIZE]; FIELD_SIZE], x: usize, y: usize) -> usize {
    if x == 0 || y == 0 || x == FIELD_SIZE - 1 || y == FIELD_SIZE - 1 {
        return 0;
    }

    let this_height = input[y][x];

    // find the tree that blocks visibility in any direction.
    let blk_l = (0..x)
        .rev()
        .find(|check_x| input[y][*check_x] >= this_height);
    let blk_r = (x + 1..FIELD_SIZE).find(|check_x| input[y][*check_x] >= this_height);

    let blk_u = (0..y)
        .rev()
        .find(|check_y| input[*check_y][x] >= this_height);
    let blk_d = (y + 1..FIELD_SIZE).find(|check_y| input[*check_y][x] >= this_height);

    // if there's no blocker, count to the edge of the map (the number of trees checked)
    let score_left = x - blk_l.unwrap_or(0);
    let score_right = blk_r.unwrap_or(FIELD_SIZE - 1) - x;

    let score_up = y - blk_u.unwrap_or(0);
    let score_down = blk_d.unwrap_or(FIELD_SIZE - 1) - y;

    score_left * score_right * score_up * score_down
}
