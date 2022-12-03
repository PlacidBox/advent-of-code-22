use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn score_for(ch: u8) -> usize {
    dbg!(ch);
    match ch {
        b'a'..=b'z' => (ch - b'a' + 1).into(),
        b'A'..=b'Z' => (ch - b'A' + 27).into(),
        // 'A'u8 .. 'Z'u8 => 0,
        _ => panic!("unexpected char"),
    }
}

fn score_for_line(line: &[u8]) -> usize {
    let (l, r) = line.split_at(line.len() / 2);
    for needle in l {
        if r.contains(needle) {
            return score_for(*needle);
        }
    }
    panic!("couldn't find duplicate character in line!");
}

fn main() {
    let input = File::open("input-day-3.txt").unwrap();
    let lines = BufReader::new(input).lines();

    let mut score = 0;

    for line in lines {
        // there's EXACTLY one dupe per character per line, but that one character may be duplicated
        // multiple times. it's all ascii, too, so working with bytes is easier.
        let line = line.unwrap();
        score += score_for_line(line.as_bytes());
    }

    println!("total score: {}", score);
}
