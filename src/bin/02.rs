advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = advent_of_code::file_util::split_to_nested_vec(input);
    // brute force it baby
    // There's a "cleaner" way to do this with another map on the reports but... idc
    let mut sum = 0;
    'outer: for report in reports {
        let mut increasing = false;
        let mut prev = report[0];
        for (idx, val) in report.iter().enumerate() {
            if idx == 1 && val > &prev {
                increasing = true;
            }
            if val > &prev && !increasing {
                continue 'outer;
            }
            if val < &prev && increasing {
                continue 'outer;
            }
            if idx != 0 && (val.abs_diff(prev) < 1 || val.abs_diff(prev) > 3) {
                continue 'outer;
            }
            prev = *val;
        }
        sum += 1;
        //if report
        //    .iter()
        //    .zip(report.iter().skip(1))
        //    .map(|(&x, &y)| (x > y && (x.abs_diff(y) >= 1 && x.abs_diff(y) <= 3)) || (x < y && (x.abs_diff(y) >= 1 && x.abs_diff(y) <= 3)))
        //    .collect::<Vec<bool>>()
        //    .iter()
        //    .all(|r| *r == true) {
        //    sum += 1;
        //}
    }
    Some(sum)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
