use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FIELD_WIDTH: usize = 81;
const FIELD_HEIGHT: usize = 41;

struct Field {
    // everything in row major
    heights: [[u8; FIELD_WIDTH]; FIELD_HEIGHT],
    end: (usize, usize),
}

impl Field {
    fn load() -> Self {
        let input = File::open("input.txt").unwrap();
        let lines = BufReader::new(input).lines();

        let mut heights = [[0; FIELD_WIDTH]; FIELD_HEIGHT];
        let mut end = (0, 0);

        for (y, line) in lines.enumerate() {
            let line = line.unwrap();
            let line = line.as_bytes();
            assert!(line.len() == FIELD_WIDTH);

            for (x, height) in line.iter().enumerate() {
                let height = match *height {
                    b'S' => b'a',
                    b'E' => {
                        end = (y, x);
                        b'z'
                    }
                    other => other,
                };

                heights[y][x] = height;
            }
        }

        Self { heights, end }
    }
}

fn main() {
    let field = Field::load();

    // track distance from start. everything begins at unknown/255, and shrinks as we process
    // points.
    let mut distances = [[u16::MAX; FIELD_WIDTH]; FIELD_HEIGHT];
    let mut to_process = vec![];

    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field.heights[y][x] == b'a' {
                distances[y][x] = 0;
                to_process.push((y, x));
            }
        }
    }

    while let Some(point) = to_process.pop() {
        // check adjacent tiles if they're the same height, or higher. if they're more than 1
        // further away from the current point, set them to current distance+1, and queue them to
        // have their neighbours updated. this is a basic flood fill, i suppose.
        let current_dist = distances[point.0][point.1];
        let current_height = field.heights[point.0][point.1];

        if point.0 != 0 {
            let tgt = (point.0 - 1, point.1);
            let tgt_height = field.heights[tgt.0][tgt.1];
            let tgt_dist = distances[tgt.0][tgt.1];

            let height_ok = tgt_height <= current_height + 1;
            if height_ok && tgt_dist > current_dist + 1 {
                distances[tgt.0][tgt.1] = current_dist + 1;
                to_process.push(tgt);
            }
        }

        if point.0 != FIELD_HEIGHT - 1 {
            let tgt = (point.0 + 1, point.1);
            let tgt_height = field.heights[tgt.0][tgt.1];
            let tgt_dist = distances[tgt.0][tgt.1];

            let height_ok = tgt_height <= current_height + 1;
            if height_ok && tgt_dist > current_dist + 1 {
                distances[tgt.0][tgt.1] = current_dist + 1;
                to_process.push(tgt);
            }
        }

        if point.1 != 0 {
            let tgt = (point.0, point.1 - 1);
            let tgt_height = field.heights[tgt.0][tgt.1];
            let tgt_dist = distances[tgt.0][tgt.1];

            let height_ok = tgt_height <= current_height + 1;
            if height_ok && tgt_dist > current_dist + 1 {
                distances[tgt.0][tgt.1] = current_dist + 1;
                to_process.push(tgt);
            }
        }

        if point.1 != FIELD_WIDTH - 1 {
            let tgt = (point.0, point.1 + 1);
            let tgt_height = field.heights[tgt.0][tgt.1];
            let tgt_dist = distances[tgt.0][tgt.1];

            let height_ok = tgt_height <= current_height + 1;
            if height_ok && tgt_dist > current_dist + 1 {
                distances[tgt.0][tgt.1] = current_dist + 1;
                to_process.push(tgt);
            }
        }
    }

    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            print!("{0}{1:>3}", field.heights[y][x] as char, distances[y][x]);
        }

        println!();
    }

    println!("{}", distances[field.end.0][field.end.1]);
}
