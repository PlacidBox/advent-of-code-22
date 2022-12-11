type Item = i64; // hit an overflow using i32, thanks rust.

struct Monkey {
    items: Vec<Item>,
    worry_op: fn(Item) -> Item,
    test_divisor: Item,
    throw_to_on_pass: usize,
    throw_to_on_fail: usize,
    inspection_count: usize,
}

const ROUND_COUNT: i32 = 20;

fn main() {
    let mut monkeys = [
        Monkey {
            items: vec![83, 62, 93],
            worry_op: |x| x * 17,
            test_divisor: 2,
            throw_to_on_pass: 1,
            throw_to_on_fail: 6,
            inspection_count: 0,
        },
        Monkey {
            items: vec![90, 55],
            worry_op: |x| x + 1,
            test_divisor: 17,
            throw_to_on_pass: 6,
            throw_to_on_fail: 3,
            inspection_count: 0,
        },
        Monkey {
            items: vec![91, 78, 80, 97, 79, 88],
            worry_op: |x| x + 3,
            test_divisor: 19,
            throw_to_on_pass: 7,
            throw_to_on_fail: 5,
            inspection_count: 0,
        },
        Monkey {
            items: vec![64, 80, 83, 89, 59],
            worry_op: |x| x + 5,
            test_divisor: 3,
            throw_to_on_pass: 7,
            throw_to_on_fail: 2,
            inspection_count: 0,
        },
        Monkey {
            items: vec![98, 92, 99, 51],
            worry_op: |x| x * x,
            test_divisor: 5,
            throw_to_on_pass: 0,
            throw_to_on_fail: 1,
            inspection_count: 0,
        },
        Monkey {
            items: vec![68, 57, 95, 85, 98, 75, 98, 75],
            worry_op: |x| x + 2,
            test_divisor: 13,
            throw_to_on_pass: 4,
            throw_to_on_fail: 0,
            inspection_count: 0,
        },
        Monkey {
            items: vec![74],
            worry_op: |x| x + 4,
            test_divisor: 7,
            throw_to_on_pass: 3,
            throw_to_on_fail: 2,
            inspection_count: 0,
        },
        Monkey {
            items: vec![68, 64, 60, 68, 87, 80, 82],
            worry_op: |x| x * 19,
            test_divisor: 11,
            throw_to_on_pass: 4,
            throw_to_on_fail: 5,
            inspection_count: 0,
        },
    ];

    for _ in 0..ROUND_COUNT {
        round(&mut monkeys);
    }

    for (i, m) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times", i, m.inspection_count);
    }
}

fn round(monkeys: &mut [Monkey; 8]) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            // there's no behavious change here if we pop from the front or the back, so do the
            // back to make it a bit simpler computationally.
            let mut cur_item = monkeys[i].items.pop().unwrap();
            // monkey inspects item and worry level increases
            monkeys[i].inspection_count += 1;
            cur_item = (monkeys[i].worry_op)(cur_item);
            // worry level is divided by 3
            cur_item = cur_item / 3;
            // monkey gets bored, throws.
            let passes_test = (cur_item % monkeys[i].test_divisor) == 0;
            let throws_to = if passes_test {
                monkeys[i].throw_to_on_pass
            } else {
                monkeys[i].throw_to_on_fail
            };

            monkeys[throws_to].items.push(cur_item);
        }
    }
}
