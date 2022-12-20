mod util;

use regex::Regex;
use std::collections::BTreeMap;
use util::next_permutation;

const INPUT: &str = include_str!("../input.txt");
const VALVE_COUNT: usize = 15;
const INITIAL_VALVE: &str = "AA";
const TIME_LIMIT: i32 = 26;

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

    // returns the total flow for visiting the sequence, and the number of entries in `sequence`
    // used before time runs out. this includes the valve that we were walking to when time ran out.
    fn total_flow_for_sequence(&self, sequence: &[u8]) -> (i32, usize) {
        if sequence.is_empty() {
            return (0, 0);
        }

        let mut flow = 0;
        let mut time_remaining = TIME_LIMIT;

        // all the initial times are under 30 minutes. this has been prechecked in the load
        time_remaining -= self.initial_costs[sequence[0] as usize] as i32;
        time_remaining -= 1; // it takes a minute to open the valve
        flow += time_remaining * self.flow_rates[sequence[0] as usize];

        let mut prev_valve = sequence[0];
        for i in 1..sequence.len() {
            let this_valve = sequence[i];

            time_remaining -= self.movement_costs[prev_valve as usize][this_valve as usize] as i32;
            time_remaining -= 1;
            if time_remaining <= 0 {
                // if i = 2, we've used 3 items total, for a count of `i+1`
                return (flow, i + 1);
            }

            flow += time_remaining * self.flow_rates[this_valve as usize];

            prev_valve = this_valve;
        }

        (flow, sequence.len())
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

// `valves` _must_ be ordred!
fn highest_flow_for_valves(puzzle: &ProcessedInput, valves: &mut [u8]) -> i32 {
    if valves.is_empty() {
        return 0;
    }

    // for all permutations of the input set, walk through it and figure out how far we get, and
    // how much flow is generated.
    //
    // Then go to the next permutation, ignoring all permutations that affect unvisited valves for
    // this solution (due to the `TIME_LIMIT`).
    let mut highest_score = 0;

    loop {
        let (score, valves_used) = puzzle.total_flow_for_sequence(valves);

        if score > highest_score {
            highest_score = score;
        }

        // Go to the next permutation that would change outcome.
        //
        // If only 5 entries in `current_perm` were used, we can skip to the next permutation that
        // modifies the front 5. next_permutation is lexographical order, so to get the next
        // permutation we reverse-sort the un-used values.
        valves[valves_used..].sort_by(|l, r| r.cmp(l));
        if !next_permutation(valves) {
            break;
        }
    }

    highest_score
}

fn main() {
    let puzzle = ProcessedInput::load();

    let mut highest_flow = 0;

    // 15 puzzle inputs, so 15 bits available. iterate through all possibilities and track the best
    let mut input_l = Vec::with_capacity(VALVE_COUNT);
    let mut input_r = Vec::with_capacity(VALVE_COUNT);
    for i in 0..2_u16.pow(VALVE_COUNT as u32) {
        for v in 0u8..VALVE_COUNT as u8 {
            let mask = 1 << v;
            if i & mask == 0 {
                input_l.push(v);
            } else {
                input_r.push(v);
            }
        }

        let flow_l = highest_flow_for_valves(&puzzle, input_l.as_mut_slice());
        let flow_r = highest_flow_for_valves(&puzzle, input_r.as_mut_slice());

        let best_flow = flow_l + flow_r;
        if best_flow > highest_flow {
            // TODO: get out the permutation that was 'best' here. it's not needed for the answer,
            //      but it'd be nice to see.
            highest_flow = best_flow;
            println!(
                "best flow {} found for inputs:\n  {:?}\n  {:?}",
                best_flow,
                input_l.as_slice(),
                input_r.as_slice()
            );
        }

        input_l.clear();
        input_r.clear();
    }

    println!("Total flow is {}", highest_flow);
}
