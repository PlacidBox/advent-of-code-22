use serde_json::Value;
use std::{
    cmp::Ord,
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut lines = BufReader::new(input).lines();

    let mut pair_number = 1; // packet numbering starts at 1
    let mut score = 0;

    while let Some((l, r)) = read_input(&mut lines) {
        println!("== Pair {} ==", pair_number);
        match value_cmp(&l, &r) {
            Ordering::Less => {
                println!(
                    "Left side is smaller. Adding {} to score {}, for {}\n",
                    pair_number,
                    score,
                    score + pair_number
                );
                score += pair_number;
            }
            Ordering::Equal => panic!(),
            Ordering::Greater => println!("Right side is smaller\n"),
        }

        pair_number += 1;
    }

    println!("Final score: {}", score);
}

fn read_input(input: &mut Lines<BufReader<File>>) -> Option<(Value, Value)> {
    let l_in = input.next()?.unwrap();
    let r_in = input.next().unwrap().unwrap();
    let _ignored = input.next(); // no last line at EOF, don't check the option

    let l: Value = serde_json::from_str(&l_in).unwrap();
    let r: Value = serde_json::from_str(&r_in).unwrap();

    Some((l, r))
}

fn value_cmp(l: &Value, r: &Value) -> Ordering {
    println!("Compare\n    {}\n  vs\n    {}\n", l, r);
    match (l, r) {
        (Value::Array(l), Value::Array(r)) => array_cmp(l, r),
        (Value::Number(l), Value::Number(r)) => {
            Ord::cmp(&l.as_i64().unwrap(), &r.as_i64().unwrap())
        }
        (Value::Array(l), Value::Number(r)) => array_num_cmp(l, r),
        (Value::Number(l), Value::Array(r)) => num_array_cmp(l, r),
        (l, r) => panic!("unexpeceted types for l = {} r = {}", l, r),
    }
}

fn num_array_cmp(l: &serde_json::Number, r: &[Value]) -> Ordering {
    let l_list = [Value::Number(l.clone())];
    array_cmp(&l_list, r)
}

fn array_num_cmp(l: &[Value], r: &serde_json::Number) -> Ordering {
    let r_list = [Value::Number(r.clone())];
    array_cmp(&l, &r_list)
}

fn array_cmp(l: &[Value], r: &[Value]) -> Ordering {
    let mut l = l.iter();
    let mut r = r.iter();

    loop {
        let this_res = match (l.next(), r.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(l_item), Some(r_item)) => value_cmp(l_item, r_item),
        };

        match this_res {
            Ordering::Equal => (), // next iter
            other => return other,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    fn str_cmp(l: &str, r: &str) -> Ordering {
        let l_item: serde_json::Value = serde_json::from_str(l).unwrap();
        let r_item: serde_json::Value = serde_json::from_str(r).unwrap();
        super::value_cmp(&l_item, &r_item)
    }

    #[test]
    fn example_one() {
        let result = str_cmp("[1,1,3,1,1]", "[1,1,5,1,1]");
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn example_two() {
        let result = str_cmp("[[1],[2,3,4]]", "[[1],4]");
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn example_three() {
        let result = str_cmp("[9]", "[[8,7,6]]");
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn example_four() {
        let result = str_cmp("[[4,4],4,4]", "[[4,4],4,4,4]");
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn example_five() {
        let result = str_cmp("[7, 7, 7, 7]", "[7, 7, 7]");
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn example_six() {
        let result = str_cmp("[]", "[3]");
        assert_eq!(result, Ordering::Less);
    }

    #[test]
    fn example_seven() {
        let result = str_cmp("[[[]]]", "[[]]");
        assert_eq!(result, Ordering::Greater);
    }

    #[test]
    fn example_eight() {
        let result = str_cmp("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(result, Ordering::Greater);
    }
}
