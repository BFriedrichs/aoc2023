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
    println!("{}", solve("12 red, 13 green, 14 blue", &load_input()));
}

fn solve(game_def: &str, input: &str) -> i32 {
    let game_def_map = map_from_game(game_def);

    let lines = input.lines();
    let mut possible = 0;

    for line in lines {
        let line = line.trim();
        let mut sides = line.split(":");
        let game_desc = sides.next().unwrap();
        let game_id = game_desc.split(" ").nth(1).unwrap().parse::<i32>().unwrap();

        let mut is_possible = true;

        let games = sides.next().unwrap().split(";");
        for game in games {
            let game_as_map = map_from_game(game);

            for (color, num) in game_as_map.iter() {
                let def_num = game_def_map.get(color);
                if def_num.is_none() || def_num.unwrap() < num {
                    is_possible = false;
                    break;
                }
            }

            if !is_possible {
                break;
            }
        }

        if is_possible {
            possible += game_id;
        }
    }
    return possible;
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

        let desired = 8;
        let result = solve("12 red, 13 green, 14 blue", test_input);
        assert_eq!(result, desired);
    }
}
