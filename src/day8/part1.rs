use std::collections::HashMap;

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i32 {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().chars();
    let mut curr_dirs = dirs.clone();
    lines.next();

    let mut graph = HashMap::new();
    for line in lines {
        let mut parts = line.trim().split(" = ");
        let from = parts.next().unwrap();
        let to_tuple = parts.next().unwrap();
        let to = to_tuple[1..to_tuple.len() - 1].split(", ");
        graph.insert(from, to);
    }

    let mut jumps = 0;
    let mut current = "AAA";
    loop {
        let mut dir = curr_dirs.next();
        if dir.is_none() {
            curr_dirs = dirs.clone();
            dir = curr_dirs.next();
        }
        let next = graph.get(current).unwrap();
        if dir.unwrap() == 'L' {
            current = next.clone().nth(0).unwrap();
        } else {
            current = next.clone().nth(1).unwrap();
        }
        jumps += 1;

        if current == "ZZZ" {
            break;
        }
    }

    return jumps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let desired = 6;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
