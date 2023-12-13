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
        } else {
            next_pos = (curr_pos.0 - 1, curr_pos.1);
            allowed_symbols = ['|', 'F', '7'];
        }
    } else if symbol == '-' {
        if prev_pos.1 < curr_pos.1 {
            next_pos = (curr_pos.0, curr_pos.1 + 1);
            allowed_symbols = ['-', 'J', '7'];
        } else {
            next_pos = (curr_pos.0, curr_pos.1 - 1);
            allowed_symbols = ['-', 'L', 'F'];
        }
    } else if symbol == '7' {
        if prev_pos.1 < curr_pos.1 {
            next_pos = (curr_pos.0 + 1, curr_pos.1);
            allowed_symbols = ['|', 'J', 'L'];
        } else {
            next_pos = (curr_pos.0, curr_pos.1 - 1);
            allowed_symbols = ['-', 'F', 'L'];
        }
    } else if symbol == 'J' {
        if prev_pos.1 < curr_pos.1 {
            next_pos = (curr_pos.0 - 1, curr_pos.1);
            allowed_symbols = ['|', '7', 'F'];
        } else {
            next_pos = (curr_pos.0, curr_pos.1 - 1);
            allowed_symbols = ['-', 'F', 'L'];
        }
    } else if symbol == 'L' {
        if prev_pos.1 > curr_pos.1 {
            next_pos = (curr_pos.0 - 1, curr_pos.1);
            allowed_symbols = ['|', 'F', '7'];
        } else {
            next_pos = (curr_pos.0, curr_pos.1 + 1);
            allowed_symbols = ['-', 'J', '7'];
        }
    } else if symbol == 'F' {
        if prev_pos.1 > curr_pos.1 {
            next_pos = (curr_pos.0 + 1, curr_pos.1);
            allowed_symbols = ['|', 'L', 'J'];
        } else {
            next_pos = (curr_pos.0, curr_pos.1 + 1);
            allowed_symbols = ['-', 'J', '7'];
        }
    } else if symbol == '.' {
        return None;
    } else {
        panic!("Unknown symbol: {}", symbol);
    }

    println!("{:?}", path.len());
    println!("{:?} {:?}, {} -> ({}, {})", prev_pos, curr_pos, symbol, next_pos.0, next_pos.1);

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

    let coord_y = (pos_s / (matrix.len() + 1)) as i32;
    let coord_x = (pos_s % (matrix.len() + 1)) as i32;

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
        return ((result.len() - 1) / 2) as i32;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";

        let desired = 4;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }

    #[test]
    fn complex() {
        let test_input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

        let desired = 8;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
