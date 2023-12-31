
fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i32 {
    let lines = input.lines(); // trim whitespace
    let mut sum = 0;
    let base: i32 = 2;

    for line in lines {
        let numbers = line.split(":").nth(1).unwrap();
        let mut split = numbers.split("|");

        let winning_nums = split.next().unwrap().trim().split(" ");
        let has_nums = split.next().unwrap().trim().split(" ");

        let mut set_of_matches = Vec::new();

        for winning_num in winning_nums {
            if winning_num == "" {
                continue;
            }
            for has_num in has_nums.clone() {
                if has_num == "" {
                    continue;
                }
                if winning_num == has_num {
                    if !set_of_matches.contains(&winning_num) {
                        set_of_matches.push(winning_num);
                    }
                }
            }
        }

        let matches = set_of_matches.len();

        if matches != 0 {
            sum += 1 * base.pow((matches - 1).try_into().unwrap());
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let desired = 13;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
