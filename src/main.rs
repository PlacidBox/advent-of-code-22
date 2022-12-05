use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // i've decided to move the initial container configuration in to code, seems easier than
    // parsing.
    // [V]     [B]                     [C]
    // [C]     [N] [G]         [W]     [P]
    // [W]     [C] [Q] [S]     [C]     [M]
    // [L]     [W] [B] [Z]     [F] [S] [V]
    // [R]     [G] [H] [F] [P] [V] [M] [T]
    // [M] [L] [R] [D] [L] [N] [P] [D] [W]
    // [F] [Q] [S] [C] [G] [G] [Z] [P] [N]
    // [Q] [D] [P] [L] [V] [D] [D] [C] [Z]
    //  1   2   3   4   5   6   7   8   9
    // we'll use base 0, and remember to fix when we read the instructions
    let mut stacks = vec![
        vec!['Q', 'F', 'M', 'R', 'L', 'W', 'C', 'V'],
        vec!['D', 'Q', 'L'],
        vec!['P', 'S', 'R', 'G', 'W', 'C', 'N', 'B'],
        vec!['L', 'C', 'D', 'H', 'B', 'Q', 'G'],
        vec!['V', 'G', 'L', 'F', 'Z', 'S'],
        vec!['D', 'G', 'N', 'P'],
        vec!['D', 'Z', 'P', 'V', 'F', 'C', 'W'],
        vec!['C', 'P', 'D', 'M', 'S'],
        vec!['Z', 'N', 'W', 'T', 'V', 'M', 'P', 'C'],
    ];

    let input = File::open("input-day-5.txt").unwrap();
    let lines: std::io::Result<Vec<String>> = BufReader::new(input).lines().collect();

    for line in lines.unwrap() {
        // each line is 'move <count> from <src> to <dst>', like 'move 1 from 9 to 2'
        let vals: Vec<&str> = line.split(' ').collect();

        let count: usize = vals[1].parse().unwrap();
        let source: usize = vals[3].parse::<usize>().unwrap() - 1;
        let dest: usize = vals[5].parse::<usize>().unwrap() - 1;

        let src = &mut stacks[source];
        let mut moved = src.split_off(src.len() - count);
        stacks[dest].append(&mut moved);
    }

    dbg!(stacks);
}
