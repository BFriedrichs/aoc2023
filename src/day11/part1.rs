fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input()));
}

fn solve(input: &str) -> i32 {
    let matrix = input.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut rows_to_insert = Vec::new();
    let mut cols_to_insert = Vec::new();

    for i in 0..matrix.len() {
        let mut row_is_empty = true;
        for j in 0..matrix[0].len() {
            if matrix[i][j] != '.' {
                row_is_empty = false;
                break;
            }
        }
        if row_is_empty {
            rows_to_insert.push(i);
        }
    }

    for j in 0..matrix[0].len() {
        let mut col_is_empty = true;
        for i in 0..matrix.len() {
            if matrix[i][j] != '.' {
                col_is_empty = false;
                break;
            }
        }
        if col_is_empty {
            cols_to_insert.push(j);
        }
    }

    println!("{:?}", rows_to_insert);

    let mut new_matrix = matrix.clone();
    for i in 0..rows_to_insert.len() {
        let mut row = Vec::new();
        for _j in 0..new_matrix[0].len() {
            row.push('.');
        }
        new_matrix.insert(rows_to_insert[i] + i, row);
    }

    println!("{:?}", cols_to_insert);

    for i in 0..cols_to_insert.len() {
        for j in 0..new_matrix.len() {
            new_matrix[j].insert(cols_to_insert[i] + i, '.');
        }
    }

    for i in 0..new_matrix.len() {
        for j in 0..new_matrix[0].len() {
            print!("{}", new_matrix[i][j]);
        }
        println!();
    }

    let mut coords = Vec::new();
    for i in 0..new_matrix.len() {
        for j in 0..new_matrix[0].len() {
            if new_matrix[i][j] == '#' {
                coords.push((i, j));
            }
        }
    }

    println!("{:?}", coords.len());

    let mut all_coord_pairs = Vec::new();
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            all_coord_pairs.push((coords[i], coords[j]));
        }
    }
    let mut sum = 0;

    println!("{:?}", all_coord_pairs.len());

    for pair in all_coord_pairs {
        let abs_x_diff = (pair.0).0 as i32 - (pair.1).0 as i32;
        let abs_y_diff = (pair.0).1 as i32 - (pair.1).1 as i32;

        sum += abs_x_diff.abs() + abs_y_diff.abs();
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let desired = 374;
        let result = solve(test_input);
        assert_eq!(result, desired);
    }
}
