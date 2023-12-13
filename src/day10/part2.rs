use std::collections::HashMap;

fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}


fn traverse(matrix: &Vec<Vec<char>>, path: &mut Vec<(i32, i32)>) -> Option<Vec<(i32, i32)>> {
    let curr_pos = path.last().unwrap();
    let prev_pos = path[path.len() - 2];

    let prev_symbol = matrix
        .get(prev_pos.0 as usize)
        .and_then(|row| row.get(prev_pos.1 as usize)).unwrap();
    let symbol = matrix
        .get(curr_pos.0 as usize)
        .and_then(|row| row.get(curr_pos.1 as usize));

    if symbol.is_none() {
        return None;
    }
    let symbol = *symbol.unwrap();

    let allowed_symbols;
    let next_pos;

    if symbol == '|' {
        if prev_pos.0 < curr_pos.0 {
            next_pos = (curr_pos.0 + 1, curr_pos.1);
            allowed_symbols = ['|', 'J', 'L'];
        } else if prev_pos.0 > curr_pos.0  {
            next_pos = (curr_pos.0 - 1, curr_pos.1);
            allowed_symbols = ['|', 'F', '7'];
        } else {
            return None;
        }
    } else if symbol == '-' {
        if prev_pos.1 < curr_pos.1 {
            next_pos = (curr_pos.0, curr_pos.1 + 1);
            allowed_symbols = ['-', 'J', '7'];
        } else if prev_pos.1 > curr_pos.1 {
            next_pos = (curr_pos.0, curr_pos.1 - 1);
            allowed_symbols = ['-', 'L', 'F'];
        } else {
            return None;
        }
    } else if symbol == '7' {
        if prev_pos.1 < curr_pos.1 {
            next_pos = (curr_pos.0 + 1, curr_pos.1);
            allowed_symbols = ['|', 'J', 'L'];
        } else if prev_pos.0 > curr_pos.0 {
            next_pos = (curr_pos.0, curr_pos.1 - 1);
            allowed_symbols = ['-', 'F', 'L'];
        } else {
            return None;
        }
    } else if symbol == 'J' {
        if prev_pos.1 < curr_pos.1 {
            next_pos = (curr_pos.0 - 1, curr_pos.1);
            allowed_symbols = ['|', '7', 'F'];
        } else if prev_pos.0 < curr_pos.0 {
            next_pos = (curr_pos.0, curr_pos.1 - 1);
            allowed_symbols = ['-', 'F', 'L'];
        } else {
            return None;
        }
    } else if symbol == 'L' {
        if prev_pos.1 > curr_pos.1 {
            next_pos = (curr_pos.0 - 1, curr_pos.1);
            allowed_symbols = ['|', 'F', '7'];
        } else if prev_pos.0 < curr_pos.0 {
            next_pos = (curr_pos.0, curr_pos.1 + 1);
            allowed_symbols = ['-', 'J', '7'];
        } else {
            return None;
        }
    } else if symbol == 'F' {
        if prev_pos.1 > curr_pos.1 {
            next_pos = (curr_pos.0 + 1, curr_pos.1);
            allowed_symbols = ['|', 'L', 'J'];
        } else if prev_pos.0 > curr_pos.0{
            next_pos = (curr_pos.0, curr_pos.1 + 1);
            allowed_symbols = ['-', 'J', '7'];
        } else {
            return None;
        }
    } else if symbol == '.' {
        return None;
    } else {
        panic!("Unknown symbol: {}", symbol);
    }

    let safe_symbol = matrix
        .get(next_pos.0 as usize)
        .and_then(|row| row.get(next_pos.1 as usize));
    if safe_symbol.is_none() {
        return None;
    } else if *safe_symbol.unwrap() != 'S' && !allowed_symbols.contains(safe_symbol.unwrap()) {
        return None;
    }

    path.push(next_pos);
    return Some(path.clone());
}

fn solve(input: &str) -> i32 {
    let matrix = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();


    let pos_s = input.replace(" ", "").find("S").unwrap();

    let coord_y = (pos_s / (matrix[0].len() + 1)) as i32;
    let coord_x = (pos_s % (matrix[0].len() + 1)) as i32;

    let mut path = Vec::new();
    path.push((coord_y, coord_x));

    let mut neigbour_coords = Vec::new();
    if coord_y > 0 {
        neigbour_coords.push((coord_y - 1, coord_x));
    }
    if coord_y < matrix.len() as i32 - 1 {
        neigbour_coords.push((coord_y + 1, coord_x));
    }
    if coord_x > 0 {
        neigbour_coords.push((coord_y, coord_x - 1));
    }
    if coord_x < matrix.len() as i32 - 1 {
        neigbour_coords.push((coord_y, coord_x + 1));
    }

    let mut result = None;
    for neighbour in neigbour_coords.iter() {
        if result.is_some() {
            break;
        }
        let mut n_path = path.clone();
        n_path.push(*neighbour);
        loop {
            result = traverse(&matrix, &mut n_path);
            if result.is_none() {
                result = None;
                break;
            }
            let coords = *n_path.last().unwrap();
            if matrix[coords.0 as usize][coords.1 as usize] == 'S' {
                break;
            }
        }
    }

    if result.is_some() {
        let result = result.unwrap();

        let mut dots = Vec::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let symbol = matrix[i][j];
                // if symbol == '.' {
                dots.push((i as i32, j as i32));
                // }
            }
        }

        let mut found = Vec::new();

        let mut count = 0;
        let mut c = 0;
        for dot in dots {
            c += 1;
            // print on same line as previous buffer
            if result.contains(&(dot.0, dot.1)) {
                continue;
            }
            let mut start = 'x';
            let mut inside = false;
            for x in 0..dot.1 {
                if !result.contains(&(dot.0, x)) {
                    continue;
                }
                let symbol = matrix[dot.0 as usize][x as usize];
                if symbol == '-' {
                    continue;
                }
                if start == 'L' && symbol == '7' {
                    inside = !inside;
                    start = 'x';
                } else if start == 'F' && symbol == 'J' {
                    inside = !inside;
                    start = 'x';
                } else if symbol == '|' || symbol == 'S' {
                    start = 'x';
                    inside = !inside;
                } else {
                    if symbol == 'F' || symbol == 'L' {
                        start = symbol;
                    } else {
                        start = 'x';
                    // if inside {
                    //     if symbol == '|' || symbol == 'J' || symbol == '7' || symbol == 'L' || symbol == 'F' {
                    //         inside = false;
                    //         if symbol == 'F' || symbol == 'L' {
                    //             start = symbol;
                    //         } else {
                    //             start = 'x';
                    //         }
                    //     }
                    // } else {
                    //     if symbol == '|' || symbol == 'J' || symbol == '7' || symbol == 'L' || symbol == 'F' {
                    //         inside = true;
                    //         if symbol == 'F' || symbol == 'L' {
                    //             start = symbol;
                    //         } else {
                    //             start = 'x';
                    //         }
                    //     }
                    }
                    // println!("{} {} ", get_replaced(&matrix[dot.0 as usize][0..(x+1) as usize].iter().collect::<String>()), inside);
                }
            }
            // println!("");
            if inside {
                count += 1;
                // println!("found one");
                found.push(dot);
            }
        }

        print_matrix(&matrix, &found);
        print_route(&matrix, &result, &found);
        return count;
    }

    return 0;
}

fn get_replaced(to_print: &str) -> String {
    return to_print.replace("7", "┐").replace("J", "┘").replace("L", "└").replace("F", "┌").replace("|", "│").replace("-", "─").to_string();
}

fn print_route(matrix: &Vec<Vec<char>>, route: &Vec<(i32, i32)>, highlight: &Vec<(i32, i32)>) {
    println!("{:?}", highlight);
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {

            let c = matrix[y][x];
            if highlight.contains(&((y as i32),(x as i32))) {
                print!("*");
            } else if !route.contains(&((y as i32),(x as i32))) {
                if c == '.' {
                    print!(".");
                } else {
                    print!(" ");
                }
            } else {
                if c == '7' {
                    print!("┐");
                } else if c == 'J' {
                    print!("┘");
                } else if c == 'L' {
                    print!("└");
                } else if c == 'F' {
                    print!("┌");
                } else if c == '|' {
                    print!("│");
                } else if c == '-' {
                    print!("─");
                } else {
                    print!("{}", c);
                }
            }
        }
        println!("");
    }
    println!("");
}

fn print_matrix(matrix: &Vec<Vec<char>>, highlight: &Vec<(i32, i32)>) {
    println!("{:?}", highlight);
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if highlight.contains(&((y as i32),(x as i32))) {
                print!("*");
            } else {
                let c = matrix[y][x];
                if c == '7' {
                    print!("┐");
                } else if c == 'J' {
                    print!("┘");
                } else if c == 'L' {
                    print!("└");
                } else if c == 'F' {
                    print!("┌");
                } else if c == '|' {
                    print!("│");
                } else if c == '-' {
                    print!("─");
                } else {
                    print!("{}", c);
                }
            }
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn works() {
    //     let test_input = "...........
    //     .S-------7.
    //     .|F-----7|.
    //     .||.....||.
    //     .||.....||.
    //     .|L-7.F-J|.
    //     .|..|.|..|.
    //     .L--J.L--J.
    //     ...........";

    //     let desired = 5;
    //     let result = solve(test_input);
    //     assert_eq!(result, desired);
    // }

    // #[test]
    // fn works2() {
    //     let test_input = "...........
    //     .S-------7.
    //     .|F-----7|.
    //     .||.....||.
    //     .||.....||.
    //     .|L--7F-J|.
    //     .|...||..|.
    //     .L---JL--J.
    //     ...........";

    //     let desired = 6;
    //     let result = solve(test_input);
    //     assert_eq!(result, desired);
    // }

    #[test]
    fn works() {
        let test_input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

        let desired = 18;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }

    // #[test]
    // fn complex() {
    //     let test_input = ".F----7F7F7F7F-7....
    //     .|F--7||||||||FJ....
    //     .||.FJ||||||||L7....
    //     FJL7L7LJLJ||LJ.L-7..
    //     L--J.L7...LJS7F-7L7.
    //     ....F-J..F7FJ|L7L7L7
    //     ....L7.F7||L7|.L7L7|
    //     .....|FJLJ|FJ|F7|.LJ
    //     ....FJL-7.||.||||...
    //     ....L---J.LJ.LJLJ...";

    //     let desired = 9;
    //     let result = solve(test_input);
    //     assert_eq!(result, desired);
    // }
}
