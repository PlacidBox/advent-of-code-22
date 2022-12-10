use std::{
    cmp::Ordering::*,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut tail_visited = HashSet::new();

    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    const ROPE_LEN: usize = 10;
    let mut rope = [(0, 0); ROPE_LEN];

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split(' ');

        let direction = parts.next().unwrap();
        let steps: i32 = parts.next().unwrap().parse().unwrap();
        assert!(steps > 0);

        for _ in 0..steps {
            let head = &mut rope[0];
            *head = move_dir(*head, direction);

            for off in 0..ROPE_LEN - 1 {
                let move_to = rope[off];
                let tail = &mut rope[off + 1];
                *tail = move_tail(*tail, move_to);
            }

            tail_visited.insert(rope[ROPE_LEN - 1]);
        }
    }

    println!("covered: {}", tail_visited.len());
}

fn move_dir(head: (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        _ => panic!("unknown direction"),
    }
}

fn move_tail(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    // if touching, no op. touching is defined as being within '1' on both axis
    let touching_x = tail.0.abs_diff(head.0) <= 1;
    let touching_y = tail.1.abs_diff(head.1) <= 1;
    if touching_x && touching_y {
        return tail;
    };

    let dx = tail.0.cmp(&head.0);
    let dy = tail.1.cmp(&head.1);

    let mut new_tail = tail;
    match dx {
        Less => new_tail.0 += 1,
        Equal => (),
        Greater => new_tail.0 -= 1,
    }

    match dy {
        Less => new_tail.1 += 1,
        Equal => (),
        Greater => new_tail.1 -= 1,
    }

    new_tail
}
