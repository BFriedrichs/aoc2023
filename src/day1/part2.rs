use std::collections::HashMap;

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

    let mut num_map = HashMap::new();
    num_map.insert("one", "1");
    num_map.insert("two", "2");
    num_map.insert("three", "3");
    num_map.insert("four", "4");
    num_map.insert("five", "5");
    num_map.insert("six", "6");
    num_map.insert("seven", "7");
    num_map.insert("eight", "8");
    num_map.insert("nine", "9");

    for line in lines {
        let line = line.trim();
        let mut first_digit: Option<String> = None;
        let mut last_digit: Option<String> = None;

        let line_len = line.len();
        for i in 0..line_len {
            if first_digit.is_some() && last_digit.is_some() {
                break;
            }
            if first_digit.is_none() {
                let first_char = line.chars().nth(i).unwrap();
                if first_char.is_digit(10) {
                    first_digit = Some(first_char.to_string());
                } else {
                    for j in 3..6 {
                        if i + j > line_len {
                            break;
                        }
                        let slice = &line[i..i + j];
                        let lookup = num_map.get(slice);
                        if lookup.is_some() {
                            first_digit = Some(lookup.unwrap().to_string());
                            break;
                        }
                    }
                }
            }
            if last_digit.is_none() {
                let last_char: char = line.chars().nth(line_len - i - 1).unwrap();
                if last_char.is_digit(10) {
                    last_digit = Some(last_char.to_string());
                } else {
                    for j in 3..6 {
                        if i + j > line_len {
                            break;
                        }
                        let slice = &line[line_len - i - j..line_len - i];
                        let lookup = num_map.get(slice);
                        if lookup.is_some() {
                            last_digit = Some(lookup.unwrap().to_string());
                            break;
                        }
                    }
                }
            }
        }

        let mut first_and_last = first_digit.unwrap();
        first_and_last.push_str(&last_digit.unwrap());

        sum += first_and_last.parse::<i32>().unwrap();
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let desired = 281;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }

    #[test]
    fn another() {
        let test_input = "jtpmfoureightvtjmlshbfour6nvjkqnddp3";

        let desired = 43;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }

    #[test]
    fn third() {
        let test_input = "58fivenine1nmhjdzlctj8fplnhmtwonexh";

        let desired = 51;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
