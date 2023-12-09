use std::collections::HashMap;

pub fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i64 {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().chars();
    let mut curr_dirs = dirs.clone();
    lines.next();

    let mut starts = Vec::new();
    let mut all_zs = Vec::new();

    let mut graph = HashMap::new();
    for line in lines {
        let mut parts = line.trim().split(" = ");
        let from = parts.next().unwrap();
        let to_tuple = parts.next().unwrap();
        let to = to_tuple[1..to_tuple.len() - 1].split(", ");

        if from.ends_with("A") {
            starts.push(from);
        }
        if from.ends_with("Z") {
            all_zs.push(from);
        }

        graph.insert(from, to);
    }

    let mut all_steps = Vec::new();

    for start in starts {
        let mut s_start = start;
        curr_dirs = dirs.clone();

        let mut steps = 0;
        loop {
            let mut dir = curr_dirs.next();
            if dir.is_none() {
                curr_dirs = dirs.clone();
                dir = curr_dirs.next();
            }

            let choice = graph.get(s_start).unwrap();
            let next;
            if dir.unwrap() == 'L' {
                next = choice.clone().nth(0).unwrap();
            } else {
                next = choice.clone().nth(1).unwrap();
            }
            steps += 1;
            if next.ends_with("Z") {
                break;
            }
            s_start = next;
        }
        all_steps.push(steps);
    }

    println!("{:?}", all_steps);
    let result = lcm(&all_steps);

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let desired = 6;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
