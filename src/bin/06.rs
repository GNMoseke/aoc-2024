use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start) = parse_to_grid(input);
    let (visited, _) = walk_grid(grid, start);
    Some(visited.len() as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    // brute force every position on the guard's path but store direction as well
    // if we repeat direction when adding a new obstacle we've found a loop
    let (grid, start) = parse_to_grid(input);
    let (mut valid_visited, _) = walk_grid(grid.clone(), start);
    let mut sum = 1;
    valid_visited.remove(&(start.0, start.1));
    for path_loc in valid_visited {
        let mut new_grid = grid.clone();
        new_grid[path_loc.0][path_loc.1] = false;
        let (_, loop_found) = walk_grid(new_grid, start);
        if loop_found {
            sum += 1;
        }
    }
    Some(sum)
}

// set of traversed gridpoints, flag if we hit a loop
fn walk_grid(
    grid: Vec<Vec<bool>>,
    start: (usize, usize, Direction),
) -> (HashSet<(usize, usize)>, bool) {
    let mut solved = false;
    let mut looped = false;
    // store directions in here as well, if we repeat a direction on a gridpoint we've looped
    let mut visited = HashMap::<(usize, usize), Vec<Direction>>::new();
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
                    let dirs_this_pos = visited
                        .entry((position.0, position.1))
                        .or_insert_with(|| vec![]);
                    if dirs_this_pos.contains(&position.2) {
                        looped = true;
                        break;
                    }
                    position = (position.0 - 1, position.1, position.2);
                    dirs_this_pos.push(position.2)
                }
            }
            Direction::Down => {
                if position.0 == grid.len() - 1 {
                    solved = true;
                } else if !grid[position.0 + 1][position.1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Left);
                } else {
                    let dirs_this_pos = visited
                        .entry((position.0, position.1))
                        .or_insert_with(|| vec![]);
                    if dirs_this_pos.contains(&position.2) {
                        looped = true;
                        break;
                    }
                    position = (position.0 + 1, position.1, position.2);
                    dirs_this_pos.push(position.2)
                }
            }
            Direction::Left => {
                if position.1 == 0 {
                    solved = true;
                } else if !grid[position.0][position.1 - 1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Up);
                } else {
                    let dirs_this_pos = visited
                        .entry((position.0, position.1))
                        .or_insert_with(|| vec![]);
                    if dirs_this_pos.contains(&position.2) {
                        looped = true;
                        break;
                    }
                    position = (position.0, position.1 - 1, position.2);
                    dirs_this_pos.push(position.2)
                }
            }
            Direction::Right => {
                if position.1 == grid[0].len() - 1 {
                    solved = true;
                } else if !grid[position.0][position.1 + 1] {
                    // specifically don't add to the sum here, just turn
                    position = (position.0, position.1, Direction::Down);
                } else {
                    let dirs_this_pos = visited
                        .entry((position.0, position.1))
                        .or_insert_with(|| vec![]);
                    if dirs_this_pos.contains(&position.2) {
                        looped = true;
                        break;
                    }
                    position = (position.0, position.1 + 1, position.2);
                    dirs_this_pos.push(position.2)
                }
            }
        }
    }
    (visited.keys().copied().collect(), looped)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        assert_eq!(result, Some(6));
    }
}
