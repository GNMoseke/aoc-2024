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
    let reports = advent_of_code::file_util::split_to_nested_vec(input);
    // brute force it baby - this is a dynamic programming problem but fuck that, the input size is
    // 10000
    let mut sum = 0;
    for report in reports {
        if check_report(report.into_iter().map(|x| x as i32).collect()) {
            sum += 1;
            continue;
        }
    }
    Some(sum)
}

fn check_report(report: Vec<i32>) -> bool {
    // brute force it with some more functional chains
    let cloned = report.clone();
    let head_iter = cloned.iter();
    let tail_iter = cloned.iter().skip(1);
    let unmodified_check_diffs: Vec<i32> = head_iter.zip(tail_iter).map(|(x, y)| y - x).collect();

    // all increasing/decreasing and within range of one another
    match (unmodified_check_diffs
        .clone()
        .into_iter()
        .clone()
        .all(|x| x < 0)
        || unmodified_check_diffs
            .clone()
            .into_iter()
            .clone()
            .all(|x| x > 0))
        && unmodified_check_diffs
            .clone()
            .into_iter()
            .all(|x| x.abs() >= 1 && x.abs() <= 3)
    {
        true => return true,
        false => {
            for (idx, _) in report.iter().enumerate() {
                let mut with_dropped = report.clone();
                with_dropped.remove(idx);
                let head_iter_d = with_dropped.iter();
                let tail_iter_d = with_dropped.iter().skip(1);
                let unmodified_check_diffs_d: Vec<i32> =
                    head_iter_d.zip(tail_iter_d).map(|(x, y)| y - x).collect();
                if (unmodified_check_diffs_d
                    .clone()
                    .into_iter()
                    .clone()
                    .all(|x| x < 0)
                    || unmodified_check_diffs_d
                        .clone()
                        .into_iter()
                        .clone()
                        .all(|x| x > 0))
                    && unmodified_check_diffs_d
                        .clone()
                        .into_iter()
                        .all(|x| x.abs() >= 1 && x.abs() <= 3)
                {
                    return true;
                }
            }
            return false;
        }
    }
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
        assert_eq!(result, Some(4));
    }
}
