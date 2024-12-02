advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second) = advent_of_code::file_util::split_to_arrays(input);
    first.sort();
    second.sort();
    let mut sum = 0;
    for i in 0..first.len() {
        sum += first[i].abs_diff(second[i]);
    }
    return Some(sum)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
