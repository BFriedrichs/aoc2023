use std::ops::Rem;

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i64 {
    let mut input = input.lines();
    let mut times = input
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .expect("Failed to parse time")
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<f64>().expect("Has to be parseable"));

    let mut distances = input
    .next()
        .unwrap()
        .split(":")
        .nth(1)
        .expect("Failed to parse distance")
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<f64>().expect("Has to be parseable"));

    let mut all_winning_times = Vec::new();

    loop {
        let time = times.next();
        let distance = distances.next();

        if time.is_none() {
            break;
        }

        let time = time.unwrap();
        let distance = distance.unwrap();

        let mut first_winning_time = (time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
        println!("First winning time: {} -> {}", first_winning_time, (time - first_winning_time) * first_winning_time);

        if first_winning_time.fract() == 0.0 {
            first_winning_time += 1.0;
        }

        let mut last_winning_time = (time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
        println!("Last winning time: {} -> {}", last_winning_time, (time - last_winning_time) * last_winning_time);

        if last_winning_time.fract() == 0.0 {
            last_winning_time -= 1.0;
        }

        let first_winning_time = first_winning_time.ceil() as i64;
        let last_winning_time = last_winning_time.floor() as i64;

        println!("{}", last_winning_time - first_winning_time + 1);
        all_winning_times.push(last_winning_time - first_winning_time + 1);
    }

    return all_winning_times.iter().product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "Time:      7  15   30
        Distance:  9  40  200";

        let desired = 288;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
