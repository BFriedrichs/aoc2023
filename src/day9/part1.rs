fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i32 {
    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let line = line.trim();
        let parts = line.split(" ").map(|part| part.parse::<i32>().unwrap()).collect::<Vec<_>>();

        let mut diffs = parts;
        let mut adds = Vec::new();

        loop {
            let mut next_diffs = Vec::new();
            let mut all_equal = true;
            for i in 0..diffs.len() - 1 {
                let slice = &diffs[i..i + 2];
                let curr = slice[0];
                let next = slice[1];
                if curr != next {
                    all_equal = false;
                }
                next_diffs.push(next - curr);
            }

            adds.insert(0, *diffs.last().unwrap());
            diffs = next_diffs;
            if all_equal {
                 break;
            }
        }

        let mut iter = adds.iter();
        let mut curr = *iter.next().unwrap();
        for add in iter {
            curr = add + curr;
        }

        sum += curr;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let desired = 114;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
