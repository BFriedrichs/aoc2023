use std::ops::Range;
use std::cmp;

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn intersection(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
    let start = cmp::max(a.start, b.start);
    let end = cmp::min(a.end, b.end);
    if start > end {
        return 0..0;
    }
    return start..end;
}

fn solve(input: &str) -> i64 {
    let mut blocks = input.split("\n\n");

    let mut avail_ranges = blocks
        .next()
        .and_then(|line| {
            line.split(": ").nth(1).map(|line| {
                line.split(" ")
                    .map(|seed| seed.parse::<i64>().expect("Seed must be number"))
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|chunk| (chunk[0]..chunk[0]+chunk[1]))
                    .collect::<Vec<_>>()
            })
        })
        .expect("failed to parse seeds");

    for block in blocks {
        avail_ranges.sort_by(|a, b| a.start.cmp(&b.start));
        println!("avail_ranges: {:?}", avail_ranges);

        let mut lines = block.lines();
        lines.next(); // skip header

        let next_ranges = lines.map(|line| {
            let line = line.trim();
            let mut slice = line.split(" ").map(|num| num.parse::<i64>().expect("num must be number"));
            let to = slice.next().expect("Needs to");
            let from = slice.next().expect("Needs from");
            let range = slice.next().expect("Needs range");
            (to, from, range)
        });

        let mut next_mapped_ranges = Vec::new();

        for avail_range in avail_ranges.clone() {
            println!("used range: {:?}", avail_range);
            let mut mapped = Vec::new();
            for (to, from, range) in next_ranges.clone() {
                println!("testing range: {:?} -> {:?}", from..from+range, to..to+range);
                let intersect = intersection(&avail_range, &(from..from+range));
                if !intersect.is_empty() {
                    let offset = to - from;
                    println!("intersect: {:?} {:?} {:?}", avail_range, from..from+range, intersect);
                    println!("offset ({}): {:?} -> {:?}", offset, intersect, intersect.start + offset..intersect.end + offset);
                    mapped.push((intersect, offset));
                }
            }
            let mut mapped_clone = mapped.clone();
            mapped_clone.sort_by(|a, b| a.0.start.cmp(&b.0.start));

            if mapped_clone.len() > 0 {
                let first_mapped = mapped_clone.first().expect("No first mapped");
                if first_mapped.0.start > avail_range.start {
                    mapped.insert(0, (avail_range.start..first_mapped.0.start, 0));
                }

                if mapped_clone.len() > 1 {
                    for i in 0..mapped_clone.len() - 1 {
                        let a = &mapped_clone[i];
                        let b = &mapped_clone[i + 1];
                        if a.0.end < b.0.start {
                            mapped.insert(i + 1, (a.0.end..b.0.start, 0));
                        }
                    }
                }

                let last_mapped = mapped_clone.last().expect("No last mapped");
                if last_mapped.0.end < avail_range.end {
                    mapped.push((last_mapped.0.end..avail_range.end, 0));
                }
            } else {
                mapped.push((avail_range.clone(), 0));
            }

            for mapped_entry in mapped {
                next_mapped_ranges.push(mapped_entry.0.start + mapped_entry.1..mapped_entry.0.end + mapped_entry.1);
            }

            println!("");
        }

        avail_ranges = next_mapped_ranges.clone();
    }

    let minimum = avail_ranges.iter().map(|range| range.start).min().expect("No minimum");

    return minimum;
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

        let desired = 46;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
