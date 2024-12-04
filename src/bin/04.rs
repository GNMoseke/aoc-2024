use regex;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    // the beauty of aoc is that I can write garbage like this without feeling bad
    let grid = to_grid(input);
    let row_ct = grid.len();
    let col_ct = grid[0].len();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            // standard horiz
            if col_idx + 3 < col_ct
                && *col == 'X'
                && row[col_idx + 1] == 'M'
                && row[col_idx + 2] == 'A'
                && row[col_idx + 3] == 'S'
            {
                sum += 1
            }
            // standard vert
            if row_idx + 3 < row_ct
                && *col == 'X'
                && grid[row_idx + 1][col_idx] == 'M'
                && grid[row_idx + 2][col_idx] == 'A'
                && grid[row_idx + 3][col_idx] == 'S'
            {
                sum += 1
            }

            // standard diag
            if row_idx + 3 < row_ct
                && col_idx + 3 < col_ct
                && *col == 'X'
                && grid[row_idx + 1][col_idx + 1] == 'M'
                && grid[row_idx + 2][col_idx + 2] == 'A'
                && grid[row_idx + 3][col_idx + 3] == 'S'
            {
                sum += 1
            }

            // reverse of above
            if col_idx + 3 < col_ct
                && *col == 'S'
                && row[col_idx + 1] == 'A'
                && row[col_idx + 2] == 'M'
                && row[col_idx + 3] == 'X'
            {
                sum += 1
            }
            if row_idx + 3 < row_ct
                && *col == 'S'
                && grid[row_idx + 1][col_idx] == 'A'
                && grid[row_idx + 2][col_idx] == 'M'
                && grid[row_idx + 3][col_idx] == 'X'
            {
                sum += 1
            }
            if row_idx + 3 < row_ct
                && col_idx + 3 < col_ct
                && *col == 'S'
                && grid[row_idx + 1][col_idx + 1] == 'A'
                && grid[row_idx + 2][col_idx + 2] == 'M'
                && grid[row_idx + 3][col_idx + 3] == 'X'
            {
                sum += 1
            }

            // other diags
            if row_idx as i32 - 3 >= 0
                && col_idx + 3 < col_ct
                && *col == 'S'
                && grid[row_idx - 1][col_idx + 1] == 'A'
                && grid[row_idx - 2][col_idx + 2] == 'M'
                && grid[row_idx - 3][col_idx + 3] == 'X'
            {
                sum += 1
            }
            if row_idx as i32 - 3 >= 0
                && col_idx + 3 < col_ct
                && *col == 'X'
                && grid[row_idx - 1][col_idx + 1] == 'M'
                && grid[row_idx - 2][col_idx + 2] == 'A'
                && grid[row_idx - 3][col_idx + 3] == 'S'
            {
                sum += 1
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = Vec::new();
    for line in input.trim().split('\n') {
        res.push(line.chars().collect());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
