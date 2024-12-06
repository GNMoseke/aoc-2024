use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start) = parse_to_grid(input);
    let mut solved = false;
    // NOTE: UNIQUE positions dumbass, took me way too long to find this stupid bug
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut position = start;
    while !solved {
        match position.2 {
            Direction::Up => {
                if position.0 == 0 {
                    solved = true;
                } else if !grid[position.0 - 1][position.1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Right);
                } else {
                    visited.insert((position.0, position.1));
                    position = (position.0 - 1, position.1, position.2);
                }
            }
            Direction::Down => {
                if position.0 == grid.len() - 1 {
                    solved = true;
                } else if !grid[position.0 + 1][position.1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Left);
                } else {
                    visited.insert((position.0, position.1));
                    position = (position.0 + 1, position.1, position.2);
                }
            }
            Direction::Left => {
                if position.1 == 0 {
                    solved = true;
                } else if !grid[position.0][position.1 - 1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Up);
                } else {
                    visited.insert((position.0, position.1));
                    position = (position.0, position.1 - 1, position.2);
                }
            }
            Direction::Right => {
                if position.1 == grid[0].len() - 1 {
                    solved = true;
                } else if !grid[position.0][position.1 + 1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Down);
                } else {
                    visited.insert((position.0, position.1));
                    position = (position.0, position.1 + 1, position.2);
                }
            }
        }
    }
    Some(visited.len() as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_to_grid(input: &str) -> (Vec<Vec<bool>>, (usize, usize, Direction)) {
    // movable spaces represented by true
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut start: (usize, usize, Direction) = (0, 0, Direction::Up);
    for (rowi, line) in input.trim().split('\n').enumerate() {
        let mut row: Vec<bool> = Vec::new();
        for (coli, char) in line.trim().chars().enumerate() {
            if char == '^' {
                start = (rowi, coli, Direction::Up);
                row.push(true);
            } else if char == '#' {
                row.push(false);
            } else {
                row.push(true);
            }
        }
        grid.push(row);
    }
    (grid, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
