use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    // this is a bfs problem
    let (graph, start_positions) = parse_to_graph(input);
    let mut visited: HashMap<(u32, u32), bool> = HashMap::new();
    let mut sum = 0;
    for start_pos in start_positions {
        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        visited.insert(start_pos, true);
        queue.push_back(start_pos);
        while !queue.is_empty() {
            let curr_pos = queue.pop_front().unwrap();
            let node = graph[curr_pos.0 as usize][curr_pos.1 as usize];

            // if it's a 9, we're done with this branch
            if node == 9 {
                sum += 1;
                continue;
            }

            for v in find_valid_edges(graph.clone(), curr_pos, node) {
                if !visited.get(&v).or(Some(&false)).unwrap() {
                    visited.insert(v, true);
                    queue.push_back(v)
                }
            }
        }
        visited.clear();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut trailheads = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'0' {
                trailheads.push((x as i32, y as i32, b'0'));
            }
        }
    }

    let mut total2 = 0;
    for t in trailheads {
        let mut queue = VecDeque::new();
        queue.push_back(t);
        let mut rating = 0;
        while let Some(pos) = queue.pop_front() {
            if pos.2 == b'9' {
                rating += 1;
                continue;
            }
            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = pos.0 + d.0;
                let ny = pos.1 + d.1;
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let d = grid[ny as usize * width + nx as usize];
                    if d == pos.2 + 1 {
                        queue.push_back((nx, ny, d));
                    }
                }
            }
        }
        total2 += rating;
    }
    Some(total2)
}

fn find_valid_edges(graph: Vec<Vec<u32>>, loc: (u32, u32), elev: u32) -> Vec<(u32, u32)> {
    let mut res: Vec<(u32, u32)> = Vec::new();
    // left
    if loc.1 > 0 && graph[loc.0 as usize][(loc.1 - 1) as usize] == elev + 1 {
        res.push((loc.0, loc.1 - 1))
    }
    // up
    if loc.0 > 0 && graph[(loc.0 - 1) as usize][loc.1 as usize] == elev + 1 {
        res.push((loc.0 - 1, loc.1))
    }
    // right
    if loc.1 < (graph[loc.0 as usize].len() - 1) as u32
        && graph[loc.0 as usize][(loc.1 + 1) as usize] == elev + 1
    {
        res.push((loc.0, loc.1 + 1))
    }
    // down
    if loc.0 < (graph.len() - 1) as u32 && graph[(loc.0 + 1) as usize][loc.1 as usize] == elev + 1 {
        res.push((loc.0 + 1, loc.1))
    }
    res
}

fn parse_to_graph(input: &str) -> (Vec<Vec<u32>>, Vec<(u32, u32)>) {
    let mut graph: Vec<Vec<u32>> = Vec::new();
    let mut start_positions: Vec<(u32, u32)> = Vec::new();
    for (rowi, line) in input.trim().split('\n').enumerate() {
        let mut row: Vec<u32> = Vec::new();
        for (coli, char) in line.chars().enumerate() {
            let elev = char.to_digit(10).unwrap() as u32;
            row.push(elev);
            if elev == 0 {
                start_positions.push((rowi as u32, coli as u32));
            }
        }
        graph.push(row);
    }
    (graph, start_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let simple = "0123\n1234\n8765\n9876";
        //let simple_res = part_one(simple);
        //assert_eq!(simple_res, Some(2));

        let example_res = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(example_res, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
