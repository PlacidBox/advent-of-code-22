use regex::Regex;

const INPUT: &str = include_str!("../input.txt");
const VALVE_COUNT: usize = 15;
const INITIAL_VALVE: &str = "AA";

struct Valve {
    name: &'static str,
    flow_rate: i32,
    leads_to: Vec<&'static str>,
}

fn read_input() -> Vec<Valve> {
    let mut result = vec![];
    // Each line has the form:
    // Valve XC has flow rate=0; tunnels lead to valves YK, AM
    let re =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    for line in INPUT.lines() {
        let caps = re.captures(line).unwrap();

        let name = caps.get(1).unwrap().as_str();
        let flow_rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let leads_to_str = caps.get(3).unwrap().as_str();

        // each tunnel is separated by ", "
        let leads_to = leads_to_str.split(", ").collect();

        result.push(Valve {
            name,
            flow_rate,
            leads_to,
        });
    }

    result
}

struct ProcessedInput {
    // We only have 15 valves worth checking. Shorten their ids to be single bytes in the range 0-15
    valve_names: [&'static str; VALVE_COUNT],
    flow_rates: [i32; VALVE_COUNT],
    // prebuild 15x15 matrix of cost, in minutes, of moving between each valve.
    movement_costs: [[u8; VALVE_COUNT]; VALVE_COUNT],
    // built cost of moving from AA (our start) to each valve
    initial_costs: [u8; VALVE_COUNT],
}

impl ProcessedInput {
    fn load() -> Self {
        let all_valves = read_input();
        todo!()
    }

    fn total_flow_for_sequence(&self, sequence: &[u8; VALVE_COUNT]) -> i32 {
        todo!()
    }
}

fn main() {
    let puzzle = ProcessedInput::load();

    // for all permutations, (there's 15! of them, oof).
    //
    // There's likely a better algorithm for this, it seems like a variant of the knapsack problem
    // at first glance, but the scoring changing depending on which order you visit things is a
    // twist i don't know how to deal with.
    //
    // 15! variations is ~40 bits though, so seems computable.
    let mut current_perm = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

    let mut highest_score = 0;
    let mut highest_scoring_perm = [0; VALVE_COUNT];

    loop {
        let score = puzzle.total_flow_for_sequence(&current_perm);

        if score > highest_score {
            highest_score = score;
            highest_scoring_perm = current_perm;
        }

        if !next_permutation(&mut current_perm) {
            break;
        }
    }

    let solution_str: [&'static str; VALVE_COUNT] =
        highest_scoring_perm.map(|x| puzzle.valve_names[x as usize]);
    print!(
        "Total flow is {} for solution {:?}",
        highest_score, solution_str
    );
}

// I genuinely can't find a crate that implements this. Maybe I should make one.
fn next_permutation<T: Ord>(input: &mut [T]) -> bool {
    todo!();
    //     The following algorithm generates the next permutation lexicographically after a
    // given permutation. It changes the given permutation in-place.

    //     Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the
    //      permutation is the last permutation.
    //     Find the largest index l greater than k such that a[k] < a[l].
    //     Swap the value of a[k] with that of a[l].
    //     Reverse the sequence from a[k + 1] up to and including the final element a[n].

    // For example, given the sequence [1, 2, 3, 4] (which is in increasing order), and given that
    // the index is zero-based, the steps are as follows:

    //     Index k = 2, because 3 is placed at an index that satisfies condition of being the
    //      largest index that is still less than a[k + 1] which is 4.
    //     Index l = 3, because 4 is the only value in the sequence that is greater than 3 in
    //      order to satisfy the condition a[k] < a[l].
    //     The values of a[2] and a[3] are swapped to form the new sequence [1, 2, 4, 3].
    //     The sequence after k-index a[2] to the final element is reversed. Because only one
    //      value lies after this index (the 3), the sequence remains unchanged in this instance.
    //      Thus the lexicographic successor of the initial state is permuted: [1, 2, 4, 3].
}
