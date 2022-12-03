use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn score_for(ch: u8) -> usize {
    match ch {
        b'a'..=b'z' => (ch - b'a' + 1).into(),
        b'A'..=b'Z' => (ch - b'A' + 27).into(),
        // 'A'u8 .. 'Z'u8 => 0,
        _ => panic!("unexpected char"),
    }
}

fn main() {
    let input = File::open("input-day-3.txt").unwrap();
    let lines: std::io::Result<Vec<String>> = BufReader::new(input).lines().collect();

    let mut score = 0;

    let chars = (b'a'..=b'z').chain(b'A'..=b'Z');

    for line in lines.unwrap().as_slice().chunks_exact(3) {
        let str_1 = line[0].as_bytes();
        let str_2 = line[1].as_bytes();
        let str_3 = line[2].as_bytes();

        // we're guaranteed exactly one common item type, no more, no less.
        for ch in chars.clone() {
            if str_1.contains(&ch) && str_2.contains(&ch) && str_3.contains(&ch) {
                score += score_for(ch);
            }
        }
    }

    println!("score {}", score);
}
