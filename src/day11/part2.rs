fn load_input() -> String {
    let input = include_str!("input.txt");
    return input.to_string();
}

fn main() {
    println!("{}", solve(&load_input(), &1000000));
}

fn solve(input: &str, expansion: &i64) -> i64 {
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

    let mut coords = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '#' {
                coords.push((i, j));
            }
        }
    }

    let mut all_coord_pairs = Vec::new();
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            all_coord_pairs.push((coords[i], coords[j]));
        }
    }
    let mut sum = 0;

    for pair in all_coord_pairs {
        let mut additions = 0;

        for row in rows_to_insert.iter() {
            if (pair.0).0 > *row && (pair.1).0 < *row || (pair.0).0 < *row && (pair.1).0 > *row {
                additions += 1;
            }
        }
        for col in cols_to_insert.iter() {
            if (pair.0).1 > *col && (pair.1).1 < *col || (pair.0).1 < *col && (pair.1).1 > *col {
                additions += 1;
            }
        }

        let abs_x_diff = (pair.0).0 as i64 - (pair.1).0 as i64;
        let abs_y_diff = (pair.0).1 as i64 - (pair.1).1 as i64;

        sum += abs_x_diff.abs() + abs_y_diff.abs() + additions * expansion - additions;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
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
        let result = solve(test_input, &1);
        assert_eq!(result, desired);
    }

    #[test]
    fn ten() {
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

        let desired = 1030;
        let result = solve(test_input, &10);
        assert_eq!(result, desired);
    }

    #[test]
    fn hun() {
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

        let desired = 8410;
        let result = solve(test_input, &100);
        assert_eq!(result, desired);
    }
}
