use petgraph::graph::DiGraph;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering, pages) = input.split_once("\n\n").unwrap();
    let graph = parse_to_graph(ordering);
    let mut valid: Vec<Vec<u32>> = Vec::new();
    let mut sum = 0;
    'outer: for update_line in pages.trim().split('\n') {
        let mut updates = update_line.split(',').map(|x| x.parse::<u32>().unwrap());
        // this could be done more cleanly but I'm preferring to be explicit
        let updates_copy: Vec<u32> = updates.clone().collect();
        let mut curr = updates.next();
        while curr != None {
            let next = updates.next();
            if next == None {
                valid.push(updates_copy);
                continue 'outer;
            }
            if !graph.contains_edge(curr.unwrap().into(), next.unwrap().into()) {
                continue 'outer;
            }
            curr = next;
        }
    }
    for valid_list in valid {
        // making the assumption that all the results have an odd number of pages
        sum += valid_list[valid_list.len() / 2]
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering, pages) = input.split_once("\n\n").unwrap();
    let graph = parse_to_graph(ordering);
    let mut invalid: Vec<Vec<u32>> = Vec::new();
    let mut sum = 0;
    'outer: for update_line in pages.trim().split('\n') {
        let mut updates = update_line.split(',').map(|x| x.parse::<u32>().unwrap());
        // this could be done more cleanly but I'm preferring to be explicit
        let updates_copy: Vec<u32> = updates.clone().collect();
        let mut curr = updates.next();
        while curr != None {
            let next = updates.next();
            if next == None {
                continue 'outer;
            }
            if !graph.contains_edge(curr.unwrap().into(), next.unwrap().into()) {
                invalid.push(updates_copy);
                continue 'outer;
            }
            curr = next;
        }
    }
    for mut invalid_list in invalid {
        // fix it - functionally sort by using the graph edges as a comparator
        invalid_list.sort_by(|x, y| {
            if graph.contains_edge((*x).into(), (*y).into()) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        // making the assumption that all the results have an odd number of pages
        sum += invalid_list[invalid_list.len() / 2]
    }

    Some(sum)
}

// NOTE: I've caved and am just using a graph library instead of writing my own parser because I am
// lazy and stupid
// this is a DAG problem
#[derive(Clone, Debug)]
struct Node {
    value: u32,
    children: HashMap<u32, Node>,
}

fn parse_to_graph(input: &str) -> DiGraph<u32, ()> {
    DiGraph::<u32, ()>::from_edges(input.split('\n').map(|line| {
        line.split_once('|')
            .map(|(p, c)| (p.parse::<u32>().unwrap(), c.parse::<u32>().unwrap()))
            .unwrap()
    }))
}

fn _parse_to_graph(input: &str) -> HashMap<u32, Node> {
    let mut node_map: HashMap<u32, Node> = HashMap::new();
    //let mut non_roots: HashSet<u32> = HashSet::new();
    for line in input.split('\n') {
        let (parent, child) = line
            .split_once('|')
            .map(|(p, c)| (p.parse::<u32>().unwrap(), c.parse::<u32>().unwrap()))
            .unwrap();
        //non_roots.insert(child);
        match (node_map.clone().get_mut(&parent), node_map.get(&child)) {
            (Some(existing_parent), Some(existing_child)) => existing_parent
                .children
                .insert(existing_child.value, existing_child.clone()),
            (None, Some(existing_child)) => node_map.insert(
                parent,
                Node {
                    value: parent,
                    children: HashMap::from([(child, existing_child.clone())]),
                },
            ),
            (Some(existing_parent), None) => existing_parent.children.insert(
                child,
                Node {
                    value: child,
                    children: HashMap::new(),
                },
            ),
            (None, None) => node_map.insert(
                parent,
                Node {
                    value: parent,
                    children: HashMap::from([(
                        child,
                        Node {
                            value: child,
                            children: HashMap::new(),
                        },
                    )]),
                },
            ),
        };
    }
    node_map
    //let root = node_map
    //    .iter()
    //    .find(|(k, _)| !non_roots.contains(k))
    //    .unwrap()
    //    .1;
    //root.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
