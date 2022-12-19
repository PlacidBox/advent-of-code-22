use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const SPAWN: (i32, i32) = (500, 0);

// set of all positions that are filled. x, y. icreasing y is 'down'
fn load() -> (HashSet<(i32, i32)>, i32) {
    let mut res = HashSet::new();
    let mut max_y = 0;

    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    for line in lines {
        let line = line.unwrap();
        let mut pen_points = line.split(" -> ").map(parse_point);

        let mut start_point = pen_points.next().unwrap();
        max_y = max_y.max(start_point.1);

        for end_point in pen_points {
            max_y = max_y.max(end_point.1);
            plot_line(start_point, end_point, &mut res);
            start_point = end_point;
        }
    }

    (res, max_y)
}

fn parse_point(input: &str) -> (i32, i32) {
    let mut digits = input.split(',');
    let x = digits.next().unwrap();
    let y = digits.next().unwrap();
    assert!(digits.next() == None);

    (x.parse().unwrap(), y.parse().unwrap())
}

fn plot_line(start_point: (i32, i32), end_point: (i32, i32), res: &mut HashSet<(i32, i32)>) {
    let dx = if start_point.0 < end_point.0 {
        1
    } else if start_point.0 > end_point.0 {
        -1
    } else {
        0
    };

    let dy = if start_point.1 < end_point.1 {
        1
    } else if start_point.1 > end_point.1 {
        -1
    } else {
        0
    };

    let mut current = start_point;
    while current != end_point {
        res.insert(current);
        current.0 += dx;
        current.1 += dy;
    }

    res.insert(end_point);
}

fn main() {
    let (mut field, max_y) = load();

    let mut settled_sand = 0;
    let mut pos = SPAWN;

    while !field.contains(&SPAWN) {
        let below_l = field.contains(&(pos.0 - 1, pos.1 + 1));
        let below_m = field.contains(&(pos.0, pos.1 + 1));
        let below_r = field.contains(&(pos.0 + 1, pos.1 + 1));

        match (below_l, below_m, below_r) {
            (true, true, true) => {
                // comes to standstill, sand solidifies to block future sand
                settled_sand += 1;
                field.insert(pos);
                pos = SPAWN;
            }
            (_, false, _) => pos.1 += 1, // falls directly downwards
            (false, true, _) => {
                // prefers to move to the left, if below is blocked
                pos.0 -= 1;
                pos.1 += 1;
            }
            (true, true, false) => {
                // otherwise falls to the right
                pos.0 += 1;
                pos.1 += 1;
            }
        }

        if pos.1 == max_y + 1 {
            // resting above the infinite ground plane
            settled_sand += 1;
            field.insert(pos);
            pos = SPAWN;
        }
    }

    println!("{}", settled_sand);
}
