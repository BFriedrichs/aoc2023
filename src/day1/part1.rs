use regex::Regex;

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
    let re = Regex::new(r"[^\d]").unwrap();
    for line in lines {
        let digits = re.replace_all(line.trim(), "");
        let chars = digits.chars();
        //concatenate first and last digit
        let mut first_and_last = chars.clone().nth(0).unwrap().to_string();
        first_and_last.push(chars.last().unwrap());
        println!("{}", first_and_last);

        sum += first_and_last.parse::<i32>().unwrap();
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let desired = 142;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
