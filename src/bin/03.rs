use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let operations: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|cap| {
            let (_, [num1, num2]) = cap.extract();
            (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap())
        })
        .collect();

    let mut sum = 0;
    for op in operations.iter() {
        sum += op.0 * op.1;
    }
    return Some(sum.try_into().unwrap());
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for enabled in input.split("do()") {
        let parsable = enabled.split("don't()").next().unwrap();
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let operations: Vec<(i32, i32)> = re
            .captures_iter(parsable)
            .map(|cap| {
                let (_, [num1, num2]) = cap.extract();
                (num1.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap())
            })
            .collect();

        for op in operations.iter() {
            sum += op.0 * op.1;
        }
    }
    return Some(sum.try_into().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
