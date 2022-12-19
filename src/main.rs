// easier than parsing. list of (sensor, beacon), where each is an (x, y) pair.
const INPUT: &[((i32, i32), (i32, i32))] = &[
    ((3523437, 2746095), (3546605, 2721324)),
    ((282831, 991087), (743030, -87472)),
    ((1473740, 3283213), (1846785, 3045894)),
    ((1290563, 46916), (743030, -87472)),
    ((3999451, 15688), (3283637, -753607)),
    ((1139483, 2716286), (1846785, 3045894)),
    ((3137614, 2929987), (3392051, 3245262)),
    ((2667083, 2286333), (2126582, 2282363)),
    ((3699264, 2920959), (3546605, 2721324)),
    ((3280991, 2338486), (3546605, 2721324)),
    ((833202, 92320), (743030, -87472)),
    ((3961416, 2485266), (3546605, 2721324)),
    ((3002132, 3500345), (3392051, 3245262)),
    ((2482128, 2934657), (1846785, 3045894)),
    ((111006, 2376713), (354526, 3163958)),
    ((424237, 2718408), (354526, 3163958)),
    ((3954504, 3606495), (3392051, 3245262)),
    ((2275050, 2067292), (2333853, 2000000)),
    ((1944813, 2557878), (2126582, 2282363)),
    ((2227536, 2152792), (2126582, 2282363)),
    ((3633714, 1229193), (3546605, 2721324)),
    ((1446898, 1674290), (2333853, 2000000)),
    ((3713985, 2744503), (3546605, 2721324)),
    ((2281504, 3945638), (1846785, 3045894)),
    ((822012, 3898848), (354526, 3163958)),
    ((89817, 3512049), (354526, 3163958)),
    ((2594265, 638715), (2333853, 2000000)),
];

const TARGET_ROW: i32 = 2000000;
const BEACONS_IN_TARGET_ROW: i32 = 1;

// Forms a half-open range. so when is_entry is _true_, the cell (x_coord, ROW) is _not_ a beacon,
// if it's _false_, it could be a beacon.
//
// The edge is between `x_coord - 1` and `x_coord`
#[derive(Debug, Clone, Copy)]
struct SensorEdge {
    x_coord: i32,
    is_entry: bool,
}

fn main() {
    // convert to list of 'distances' from each sensor that can't be beacons (or is a beacon). this
    // gives us the entry and exit cell for each sensor along the row. track entering an area as +1,
    // exiting as -1, any time we're positive there can't be a non-detected-beacon.
    //
    // so sort the list of edges, traverse counting how many sensors are covered, and sum up
    // positive patches
    //
    // then remove 3 because there's 3 beacons detected in that row, and the answer doesn't include
    // those.
    let mut edges = vec![];

    for ((s_x, s_y), (b_x, b_y)) in INPUT {
        // get distance around sensor (manhatten distance)
        let dx = s_x.abs_diff(*b_x);
        let dy = s_y.abs_diff(*b_y);
        let distance = dx + dy;

        // range left/right along the target row:
        // for a sensor of distance 2:
        // ..#..
        // .B##.
        // ##S##
        // .###. <-- target row
        // ..#..
        // distance taken by vertical: 1, so 1 remaining either side at (s_x, TARGET_ROW)
        let range_to_tgt_row = s_y.abs_diff(TARGET_ROW);

        if range_to_tgt_row <= distance {
            let range = distance - range_to_tgt_row;
            let left_edge = SensorEdge {
                x_coord: s_x - range as i32,
                is_entry: true,
            };

            let right_edge = SensorEdge {
                x_coord: s_x + range as i32 + 1,
                is_entry: false,
            };

            // 4985188
            // too low

            // 4985191

            println!(
                "Sensor ({},{}), B ({},{}) covers target row from {} to {}",
                s_x, s_y, b_x, b_y, left_edge.x_coord, right_edge.x_coord
            );

            edges.push(left_edge);
            edges.push(right_edge);
        }
    }

    edges.sort_by_key(|edge| edge.x_coord);

    let mut covered_cells = 0;
    let mut entered_at = None;
    let mut coverage_count = 0;

    for edge in &edges {
        if edge.is_entry && coverage_count == 0 {
            assert_eq!(entered_at, None);
            entered_at = Some(edge.x_coord);
            println!("Coverage starts at x={}", edge.x_coord);
        }

        if !edge.is_entry && coverage_count == 1 {
            //           1    1
            // 0    5    0    5
            // ....######......
            //     ^     ^
            // entry=4    exit=10
            // distance covered = 6
            let distance_covered = edge.x_coord - entered_at.unwrap();
            covered_cells += distance_covered;
            entered_at = None;
            println!("Coverage starts at x={}", edge.x_coord);
        }

        match edge.is_entry {
            true => coverage_count += 1,
            false => coverage_count -= 1,
        }
    }

    assert_eq!(entered_at, None);
    assert_eq!(coverage_count, 0);

    println!("answer: {}", covered_cells - BEACONS_IN_TARGET_ROW);
}
