use regex::Regex;
use std::cmp;
use std::collections::HashMap;

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

    let re_symbol = Regex::new(r"[*]").unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let mut adjacency = HashMap::new();

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

        let symbols = re_symbol.find_iter(&full_box);
        for symbol_match in symbols {
            let symbol_start = symbol_match.start();
            let symbol_y_coord = y_up + symbol_start / (x_right - x_left);
            let symbol_x_coord = x_left + symbol_start % (x_right - x_left);

            let has_num = adjacency.get(&(symbol_y_coord, symbol_x_coord));
            if has_num.is_some() {
                sum += has_num.unwrap() * num.parse::<i32>().unwrap();
            } else {
                adjacency.insert((symbol_y_coord, symbol_x_coord), num.parse::<i32>().unwrap());
            }
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

        let desired = 467835;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }

    #[test]
    fn works2() {
        let test_input = "467..114..
        ..........
        ..35..633.
        ......#...
        ...759....
        ......*.+.
        ...313....
        ......755.
        ...$......
        .664.598..";

        let desired = 237567;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
