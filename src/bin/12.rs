advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    // pretty sure this is a dfs problem
    // 1. iterate grid, if node not visited start dfs from it
    // 2. during adjacency check return the num nodes that are of a diff char
    // 3. keep running total of visited & perim from adjacency check above
    // 4. cont. until end of grid reached (don't reset visited)
    // 5. sum
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
