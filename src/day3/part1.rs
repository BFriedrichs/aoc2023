use regex::Regex;
use std::cmp;

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i32 {
    let input = input.replace(" ", ""); // trim whitespace
    let lines = input.lines();
    let line_count = lines.count();

    let line_len = input.split("\n").next().unwrap().len() + 1; // account for newline

    let re_symbol = Regex::new(r"[^0-9.]").unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let mut sum = 0;
    let all_nums = re.find_iter(&input);
    for num_match in all_nums {
        let num = num_match.as_str();

        let num_start = num_match.start();
        let y_coord = num_start / line_len;
        let x_coord = num_start % line_len;

        let y_up = cmp::max(y_coord, 1) - 1;
        let y_down = cmp::min(y_coord + 1, line_count - 1);

        let x_left = cmp::max(x_coord, 1) - 1;
        let x_right = cmp::min(x_coord + num.len() + 1, line_len - 1);

        let mut full_box = String::new();

        if y_coord != 0 {
            let top_left_serialised = (y_up * line_len) + x_left;
            let top_right_serialised = (y_up * line_len) + x_right;

            let top_box = &input[top_left_serialised..top_right_serialised];
            full_box.push_str(top_box);
        }

        let mid_left_serialised = (y_coord * line_len) + x_left;
        let mid_right_serialised = (y_coord * line_len) + x_right;

        let mid_box = &input[mid_left_serialised..mid_right_serialised];
        full_box.push_str(mid_box);

        if y_down != line_count - 1 {
            let bottom_left_serialised = (y_down * line_len) + x_left;
            let bottom_right_serialised = (y_down * line_len) + x_right;

            let bottom_box = &input[bottom_left_serialised..bottom_right_serialised];
            full_box.push_str(bottom_box);
        }

        let has_symbol = re_symbol.is_match(&full_box);

        if has_symbol {
            sum += num.parse::<i32>().unwrap();
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let desired = 4361;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
