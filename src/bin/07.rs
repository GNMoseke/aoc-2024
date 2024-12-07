use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let calibrations = parse(input);
    let mut valid: HashSet<u64> = HashSet::new();
    for calib in calibrations {
        check_calib(
            *calib.1.first().unwrap(),
            calib.1[1..].to_vec(),
            calib.0,
            &mut valid,
        );
    }
    Some(valid.iter().sum())
}

fn check_calib(val: u64, remainder: Vec<u64>, target: u64, valid: &mut HashSet<u64>) {
    if remainder.len() > 0 {
        // stop recursing, we've either already solved or can no longer solve down this permutation
        if val > target || valid.contains(&target) {
            return;
        }

        let plus = val + remainder.first().unwrap();
        let mult = val * remainder.first().unwrap();

        // recurse for remaining permutations
        check_calib(plus, remainder[1..].to_vec(), target, valid);
        check_calib(mult, remainder[1..].to_vec(), target, valid);
    } else {
        // no remaining permutations to check, see if we've solved
        if val == target {
            valid.insert(target);
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut res: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.trim().split('\n') {
        let (val, inputs) = line.split_once(':').unwrap();
        res.push((
            val.parse().unwrap(),
            inputs
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect(),
        ))
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
