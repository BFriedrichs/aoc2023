use std::collections::HashMap;

fn map_from_game(game: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();
    let cubes = game.split(",");
    for cube in cubes {
        let cube = cube.trim();
        let mut split = cube.split(" ");
        let num = split.next().unwrap().parse::<i32>().unwrap();
        let color = split.next().unwrap();
        map.insert(color, num);
    }
    return map;
}

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
        let mut sides = line.split(":");
        let mut min_map = HashMap::new();

        let games = sides.nth(1).unwrap().split(";");
        for game in games {
            let game_as_map = map_from_game(game);

            for (color, num) in game_as_map.iter() {
                let min_num = min_map.get(color);
                if min_num.is_none() || min_num.unwrap() < num {
                    min_map.insert(*color, *num);
                }
            }
        }

        sum += min_map.values().product::<i32>();
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let desired = 2286;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
