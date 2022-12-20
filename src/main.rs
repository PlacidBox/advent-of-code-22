use regex::Regex;
use std::collections::BTreeMap;

const INPUT: &str = include_str!("../input.txt");
const VALVE_COUNT: usize = 15;
const INITIAL_VALVE: &str = "AA";
const TIME_LIMIT: i32 = 30;

struct ValveDetails {
    flow_rate: i32,
    leads_to: Vec<&'static str>,
}

fn read_input() -> BTreeMap<&'static str, ValveDetails> {
    let mut result = BTreeMap::new();
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

        result.insert(
            name,
            ValveDetails {
                flow_rate,
                leads_to,
            },
        );
    }

    result
}

struct ProcessedInput {
    // We only have 15 valves worth checking. Shorten their ids to be single bytes in the range 0-15
    valve_names: [&'static str; VALVE_COUNT],
    flow_rates: [i32; VALVE_COUNT],
    // built cost of moving from AA (our start) to each valve
    initial_costs: [u8; VALVE_COUNT],
    // prebuild 15x15 matrix of cost, in minutes, of moving between each valve.
    // indexed as [src][dest]
    movement_costs: [[u8; VALVE_COUNT]; VALVE_COUNT],
}

impl ProcessedInput {
    fn load() -> Self {
        // We refer to our input by the line's valve name a part while calculating our intermediates
        let all_valves = read_input();

        // Get the list of valves that are useful to open. These form the verticies of our
        // simplified puzzle graph
        let valve_names: Vec<&'static str> = all_valves
            .iter()
            .filter_map(|(&name, details)| {
                if details.flow_rate > 0 {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(valve_names.len(), VALVE_COUNT);
        let valve_names: [&'static str; VALVE_COUNT] = valve_names.try_into().unwrap();

        let flow_rates = valve_names.map(|name| all_valves[name].flow_rate);

        let initial_costs = shortest_paths(&all_valves, INITIAL_VALVE, &valve_names);
        let movement_costs =
            valve_names.map(|name| shortest_paths(&all_valves, name, &valve_names));

        // The logic in `total_flow_for_sequence` assumes that any initial step won't break our time
        // limit.
        assert_eq!(initial_costs.iter().find(|&x| *x as i32 > TIME_LIMIT), None);

        ProcessedInput {
            valve_names,
            flow_rates,
            initial_costs,
            movement_costs,
        }
    }

    fn total_flow_for_sequence(&self, sequence: &[u8; VALVE_COUNT]) -> i32 {
        let mut flow = 0;
        let mut time_remaining = TIME_LIMIT;

        // all the initial times are under 30 minutes. this has been prechecked in the load
        time_remaining -= self.initial_costs[sequence[0] as usize] as i32;
        time_remaining -= 1; // it takes a minute to open the valve
        flow += time_remaining * self.flow_rates[sequence[0] as usize];

        let mut prev_valve = sequence[0];
        for i in 1..VALVE_COUNT {
            let this_valve = sequence[i];

            time_remaining -= self.movement_costs[prev_valve as usize][this_valve as usize] as i32;
            time_remaining -= 1;
            if time_remaining <= 0 {
                break;
            }

            flow += time_remaining * self.flow_rates[this_valve as usize];

            prev_valve = this_valve;
        }

        flow
    }
}

fn shortest_paths(
    within: &BTreeMap<&'static str, ValveDetails>,
    from: &'static str,
    to_valves: &[&'static str; VALVE_COUNT],
) -> [u8; VALVE_COUNT] {
    let full_set = shortest_paths_full(within, from);
    to_valves.map(|name| full_set[name])
}

fn shortest_paths_full(
    within: &BTreeMap<&'static str, ValveDetails>,
    from: &'static str,
) -> BTreeMap<&'static str, u8> {
    let mut distances = BTreeMap::new();
    distances.insert(from, 0);
    let mut to_update = vec![from];

    while let Some(name) = to_update.pop() {
        let cur_dist = distances[name];
        // It takes 1 minute to move between all nodes
        let next_dist = cur_dist + 1;

        let leads_to = &within[name].leads_to;

        for dest in leads_to {
            let existing_dist = distances.get(dest);
            let can_shorten = existing_dist.map_or(true, |&d| next_dist < d);
            if can_shorten {
                distances.insert(dest, next_dist);
                to_update.push(&dest);
            }
        }
    }

    distances
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

    let mut permutations_checked = 0u64;

    loop {
        if permutations_checked % 100000000 == 0 {
            let solution_str: [&'static str; VALVE_COUNT] =
                current_perm.map(|x| puzzle.valve_names[x as usize]);
            println!("Checking {:?}...", solution_str);
        }
        permutations_checked += 1;

        let score = puzzle.total_flow_for_sequence(&current_perm);

        if score > highest_score {
            highest_score = score;
            highest_scoring_perm = current_perm;

            let solution_str: [&'static str; VALVE_COUNT] =
                highest_scoring_perm.map(|x| puzzle.valve_names[x as usize]);
            println!(
                "New best is {:?} with flow of {}",
                solution_str, highest_score
            );
        }

        if !next_permutation(&mut current_perm) {
            break;
        }
    }

    let solution_str: [&'static str; VALVE_COUNT] =
        highest_scoring_perm.map(|x| puzzle.valve_names[x as usize]);
    println!(
        "Total flow is {} for solution {:?}",
        highest_score, solution_str
    );
}

// I genuinely can't find a crate that implements this. Maybe I should make one.
fn next_permutation<T: Ord>(input: &mut [T]) -> bool {
    // From wikipedia:
    //     The following algorithm generates the next permutation lexicographically after a
    // given permutation. It changes the given permutation in-place.
    //     Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the
    //      permutation is the last permutation.
    //     Find the largest index l greater than k such that a[k] < a[l].
    //     Swap the value of a[k] with that of a[l].
    //     Reverse the sequence from a[k + 1] up to and including the final element a[n].

    // Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the permutation
    // is the last permutation.
    let mut k = None;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            k = Some(i);
        }
    }

    let Some(k) = k else {
        // Last permutation was reached, the input is in reverse order lexographically so reverse it
        // to get it back to sorted, and return `false` to indicate that we're done.
        input.reverse();
        return false;
    };

    // Find the largest index l greater than k such that a[k] < a[l].
    let mut l = None;
    for i in k + 1..input.len() {
        if input[k] < input[i] {
            l = Some(i);
        }
    }
    // this should be guaranteed to be non-None, since we already checked when deciding k.
    // It's something to clean up when publishing as a crate, though.
    let l = l.unwrap();

    // Swap the value of a[k] with that of a[l].
    input.swap(k, l);

    // Reverse the sequence from a[k + 1] up to and including the final element a[n].
    input[k + 1..].reverse();

    true
}

#[cfg(test)]
mod tests {
    use super::next_permutation;
    #[test]
    fn test_next_perm() {
        let mut list = [0, 1, 2];

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [0, 2, 1]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [1, 0, 2]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [1, 2, 0]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [2, 0, 1]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [2, 1, 0]);

        assert_eq!(next_permutation(&mut list), false);
        assert_eq!(list, [0, 1, 2]);
    }

    #[test]
    fn test_next_perm_dupes() {
        let mut list = ['a', 'a', 'b'];

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, ['a', 'b', 'a']);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, ['b', 'a', 'a']);

        assert_eq!(next_permutation(&mut list), false);
        assert_eq!(list, ['a', 'a', 'b']);
    }
}
