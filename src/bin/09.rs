advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_to_blocks(input);
    let ct = blocks.len() - 1;
    let mut curr_swap_idx = blocks.iter().position(|x| *x == None).unwrap();
    let mut prev_swap_idx = curr_swap_idx;
    for (idx, id) in blocks.clone().iter().rev().enumerate() {
        // done, next point to fill is after where we are. unswap the most recent change
        if curr_swap_idx < prev_swap_idx {
            blocks.swap(curr_swap_idx, prev_swap_idx);
            break;
        }
        match id {
            Some(_) => {
                blocks.swap(ct - idx, curr_swap_idx);
                // could slightly improve this by only checking forwards from the current swap idx
                // but idc
                prev_swap_idx = curr_swap_idx;
                curr_swap_idx = blocks.iter().position(|x| *x == None).unwrap();
            }
            None => continue,
        }
    }
    Some(calc_sum(&blocks))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_to_blocks(input: &str) -> Vec<Option<u64>> {
    let mut id = 0;
    let mut res: Vec<Option<u64>> = Vec::new();
    for (idx, char) in input.trim().chars().enumerate() {
        let num = char.to_digit(10).unwrap();
        // file
        if idx % 2 == 0 {
            for _ in 0..num {
                res.push(Some(id));
            }
            id += 1;
            continue;
        }
        for _ in 0..num {
            res.push(None);
        }
    }
    res
}

fn calc_sum(blocks: &Vec<Option<u64>>) -> u64 {
    let mut sum = 0;
    for (idx, val) in blocks.iter().enumerate() {
        if *val != None {
            sum += val.unwrap() * idx as u64
        }
    }
    sum
}

fn debug_print(blocks: &Vec<Option<u64>>) {
    println!(
        "{:?}",
        blocks
            .iter()
            .map(|x| match x {
                Some(val) => char::from_digit(*val as u32, 10).unwrap(),
                None => '.',
            })
            .collect::<Vec<char>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
