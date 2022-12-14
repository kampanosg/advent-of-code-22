pub mod structs;
use core::cmp::Reverse;
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap};
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let map = parse_map(file_contents);

    match part.as_str() {
        "p1" => p1(map),
        "p2" => p2(map),
        _ => println!(""),
    }
}

// Find the shortest possible path using Dijkstra's algorithm
fn p1(map: structs::AreaMap) {
    let mut shortest_path = 0;

    let start_at = map.start_at;
    let mut open = BinaryHeap::from([(Reverse(0), start_at)]);
    let mut steps = HashMap::from([(start_at, 0)]);

    while let Some((_, pos)) = open.pop() {
        if pos == map.end_at {
            shortest_path = steps.get(&pos).copied().unwrap();
            break;
        }

        let Some(neighbors) = map.graph.get(&pos) else { continue; };

        for maybe_neighbor in neighbors {
            let Some(neighbor) = maybe_neighbor else { continue; };
            let next_steps: u32 = steps.get(&pos).unwrap() + 1;
            let curr_steps: u32 = *steps.get(neighbor).unwrap_or(&u32::MAX);

            if next_steps >= curr_steps {
                continue;
            }

            open.push((Reverse(next_steps), *neighbor));
            steps.insert(*neighbor, next_steps);
        }
    }

    println!("shortest path = {}", shortest_path);
}

fn p2(map: structs::AreaMap) {
    let dhill = parse_downhill_map(map);
    let steps = shortest_steps_from_summit(dhill.clone());

    let mut shortest_path = u32::MAX;
    for (pos, steps_to_pos) in steps.iter() {
        let (row, col) = *pos;
        let structs::Hill::Hill(0) = dhill.clone().hills[row][col] else { continue; };
        shortest_path = min(shortest_path, *steps_to_pos);
    }

    println!("{}", shortest_path);
}

fn parse_map(file_contents: String) -> structs::AreaMap {
    let hills: Vec<Vec<_>> = file_contents
        .lines()
        .map(|row| row.chars().map(structs::Hill::from).collect())
        .collect();

    let mut graph = HashMap::new();

    let last_row = hills.len().saturating_sub(1);
    let last_col = hills
        .first()
        .map(|r| r.len())
        .unwrap_or_default()
        .saturating_sub(1);

    let mut start_at = (0, 0);
    let mut end_at = (0, 0);

    for (row_idx, row) in hills.iter().enumerate() {
        for (col_idx, hill) in row.iter().enumerate() {
            let mut neighbors = [None; 4];
            if row_idx > 0 && hill.can_reach(&hills[row_idx - 1][col_idx]) {
                neighbors[0] = Some((row_idx - 1, col_idx));
            }
            if col_idx > 0 && hill.can_reach(&hills[row_idx][col_idx - 1]) {
                neighbors[1] = Some((row_idx, col_idx - 1));
            }
            if row_idx < last_row && hill.can_reach(&hills[row_idx + 1][col_idx]) {
                neighbors[2] = Some((row_idx + 1, col_idx));
            }
            if col_idx < last_col && hill.can_reach(&hills[row_idx][col_idx + 1]) {
                neighbors[3] = Some((row_idx, col_idx + 1));
            }

            if let structs::Hill::Start(_) = hill {
                start_at = (row_idx, col_idx);
            }
            if let structs::Hill::End(_) = hill {
                end_at = (row_idx, col_idx);
            }
            graph.insert((row_idx, col_idx), neighbors);
        }
    }

    structs::AreaMap {
        hills,
        graph,
        start_at,
        end_at,
    }
}

fn parse_downhill_map(map: structs::AreaMap) -> structs::DownhillMap {
    let mut graph: HashMap<(usize, usize), structs::AdjacentHills> = HashMap::new();

    for (pos, neighbors) in map.graph.iter() {
        for neighbor in neighbors.iter().flatten() {
            graph
                .entry(*neighbor)
                .or_default()
                .iter_mut()
                .filter(|slot| slot.is_none())
                .take(1)
                .for_each(|slot| *slot = Some(*pos));
        }
    }

    let hills = map.hills.to_vec();
    let summit = map.end_at;

    structs::DownhillMap {
        hills,
        graph,
        summit,
    }
}

fn shortest_steps_from_summit(dhill_map: structs::DownhillMap) -> HashMap<(usize, usize), u32> {
        let start_at = dhill_map.summit;
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);
        let mut steps = HashMap::from([(start_at, 0)]);

        while let Some((_, pos)) = open.pop() {
            
            let Some(neighbors) = dhill_map.graph.get(&pos) else { continue; };
            for maybe_neighbor in neighbors {
                let Some(neighbor) = maybe_neighbor else { continue; };
                let next_steps: u32 = steps.get(&pos).unwrap() + 1;
                let curr_steps: u32 = *steps.get(neighbor).unwrap_or(&u32::MAX);
                if next_steps >= curr_steps {
                    continue;
                }
                open.push((Reverse(next_steps), *neighbor));
                steps.insert(*neighbor, next_steps);
            }
        }

        steps
    }
