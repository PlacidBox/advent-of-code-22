use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_Y: i32 = 200; // from a quick scan of the input text file
const SPAWN: (i32, i32) = (500, 0);

// set of all positions that are filled. x, y. icreasing y is 'down'
fn load() -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();

    let input = File::open("input.txt").unwrap();
    let lines = BufReader::new(input).lines();

    for line in lines {
        let line = line.unwrap();
        let mut pen_points = line.split(" -> ").map(parse_point);

        let mut start_point = pen_points.next().unwrap();
        for end_point in pen_points {
            plot_line(start_point, end_point, &mut res);
            start_point = end_point;
        }
    }

    res
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
    let mut field = load();

    let mut settled_sand = 0;
    let mut pos = SPAWN;

    while pos.1 < MAX_Y {
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
    }

    println!("{}", settled_sand);
}
