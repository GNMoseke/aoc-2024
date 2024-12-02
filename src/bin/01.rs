advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second) = advent_of_code::file_util::split_to_arrays(input);
    first.sort();
    second.sort();
    let mut sum = 0;
    for i in 0..first.len() {
        sum += first[i].abs_diff(second[i]);
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = advent_of_code::file_util::split_to_arrays(input);
    let f = second
        .iter()
        .fold(std::collections::HashMap::new(), |mut dict, val| {
            dict.entry(val).and_modify(|count| *count += 1).or_insert(1);
            return dict;
        });

    let mut sum = 0;
    for num in first.iter() {
        sum += num * f.get(num).unwrap_or(&0);
    }
    return Some(sum);
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
        assert_eq!(result, Some(31));
    }
}
