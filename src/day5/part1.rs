fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i64 {
    let mut blocks = input.split("\n\n");

    let mut avail_nums = blocks
        .next()
        .and_then(|line| {
            line.split(": ").nth(1).map(|line| {
                line.split(" ")
                    .map(|seed| seed.parse::<i64>().expect("Seed must be number"))
                    .collect::<Vec<_>>()
            })
        })
        .expect("failed to parse seeds");
    for block in blocks {
        let mut lines = block.lines();
        lines.next(); // skip header

        let ranges = lines.map(|line| {
            let line = line.trim();
            let mut slice = line.split(" ").map(|num| num.parse::<i64>().expect("num must be number"));
            let to = slice.next().expect("Needs to");
            let from = slice.next().expect("Needs from");
            let range = slice.next().expect("Needs range");
            (to, from, range)
        });

        avail_nums = avail_nums.iter().map(|num: &i64| {
            ranges.clone().find(|(_, from, range)| {
                *num >= *from && *num < *from + *range
            }).map(|(to, from, _)| {
                to + (*num - from)
            }).unwrap_or(*num)

        }).collect::<Vec<i64>>();
    }

    return *avail_nums.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";

        let desired = 35;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
