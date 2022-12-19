const INPUT: &str = include_str!("../input.txt");
const VALVE_COUNT: usize = 15;
const INITIAL_VALVE: &str = "AA";

fn main() {
    // We only have 15 valves worth checking.

    // build the set of useful valves
    let valve_names: [&'static str; VALVE_COUNT];
    let flow_rates: [i32; VALVE_COUNT];

    // prebuild 15x15 matrix of cost, in minutes, of moving between each valve.
    let movement_costs: [[u8; VALVE_COUNT]; VALVE_COUNT];

    // built cost of moving from AA (our start) to each valve
    let initial_costs: [u8; VALVE_COUNT];

    // for all permutations, (there's 15! of them, oof),
    //  add cost in minutes of going from each node, and opening (+1)
    //  add flow rate * time remaining to score
    //  if out of time, that's our score
    //  keep track of max score
    let mut current_perm = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

    let mut highest_score = 0;
    let mut highest_scoring_perm = [0; VALVE_COUNT];

    loop {
        let score = score_for_perm(&current_perm);

        if score > highest_score {
            highest_score = score;
            highest_scoring_perm = current_perm;
        }

        if !next_permutation(&mut current_perm) {
            break;
        }
    }
}

fn score_for_perm(current_perm: &[u8; VALVE_COUNT]) -> i32 {
    todo!()
}

fn next_permutation<T: Ord>(input: &mut [T]) -> bool {
    todo!();
}
