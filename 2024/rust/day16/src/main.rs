use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    isize,
    time::Instant,
    usize, vec,
};
#[derive(Eq, PartialEq)]
// yoinked from mr gippity
#[derive(Debug)]
struct Node {
    position: (isize, isize),
    rotation: (isize, isize), // [(0, -1), (1, 0), (0, 1), (-1, 0)]; // North, East, South, West
    g_cost: usize,
    path: HashSet<(isize, isize)>,
}

// To allow comparison in BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.g_cost.cmp(&self.g_cost) // Reverse for min-heap
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// end yoink

fn main() {
    let start = Instant::now();
    let input = parse_input();
    let max_len = part_one(&input);
    part_two(&input, max_len);
    println!("Time elapsed: {:.3?}", start.elapsed());
}

fn parse_input() -> Vec<Vec<char>> {
    let input: &str = include_str!("../input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();
    return map;
}

fn get_char_pos(map: &Vec<Vec<char>>, char: &char) -> (isize, isize) {
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == *char {
                return (row_i as isize, col_i as isize);
            }
        }
    }
    unreachable!();
}
fn get_neighbors(
    grid: &Vec<Vec<char>>,
    position: (isize, isize),
    path: &HashSet<(isize, isize)>,
) -> Vec<(isize, isize)> {
    let (x, y) = position;
    let mut neighbors = Vec::new();

    // Possible moves: up, down, left, right
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dx, dy) in deltas.iter() {
        let nx = x.wrapping_add(*dx) as usize;

        let ny = y.wrapping_add(*dy) as usize;

        if nx < grid.len()
            && ny < grid[0].len()
            && grid[nx][ny] != '#'
            && !path.contains(&(nx as isize, ny as isize))
        {
            neighbors.push((nx as isize, ny as isize));
        }
    }
    neighbors
}
// keeping the score for reaching all positions is key!!
fn get_neighbors_2(
    grid: &Vec<Vec<char>>,
    position: (isize, isize),
    path: &HashSet<(isize, isize)>,
    score: &usize,
    seen: &mut HashMap<(isize, isize), usize>,
) -> Vec<(isize, isize)> {
    let (x, y) = position;
    let mut neighbors = Vec::new();

    // Possible moves: up, down, left, right
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dx, dy) in deltas.iter() {
        let nx = x.wrapping_add(*dx) as usize;

        let ny = y.wrapping_add(*dy) as usize;
        let last_seen_score = *seen
            .entry((nx as isize, ny as isize))
            .or_insert(usize::MAX - 1000)
            + 1000;

        if nx < grid.len()
            && ny < grid[0].len()
            && grid[nx][ny] != '#'
            && !path.contains(&(nx as isize, ny as isize))
            && score <= &last_seen_score
        // we might step on a path where we are just about to turn
        {
            seen.insert((nx as isize, ny as isize), *score);
            neighbors.push((nx as isize, ny as isize));
        }
    }
    neighbors
}
fn part_two(input: &Vec<Vec<char>>, max_score: usize) -> usize {
    let start_pos = get_char_pos(input, &'S');
    let end_pos = get_char_pos(input, &'E');

    let mut open_list = BinaryHeap::new();
    // key: where you are
    // value: where you came from
    let mut seen: HashMap<(isize, isize), usize> = HashMap::new();

    // Push the starting node
    open_list.push(Node {
        position: start_pos,
        rotation: (0, 1),
        g_cost: 0,
        path: HashSet::from_iter(vec![start_pos]),
    });

    let mut output_cost = 0;

    let mut shortest_paths = Vec::new();
    let mut min_cost = usize::MAX;

    while let Some(current) = open_list.pop() {
        if current.g_cost > min_cost || current.g_cost > max_score {
            continue;
        }
        //println!("open list len: {:?}", open_list.len());
        //println!("current cost: {:?}", current.g_cost);

        if current.position == end_pos {
            output_cost = current.g_cost;
            if output_cost < min_cost {
                min_cost = output_cost;
                shortest_paths.clear();
            }
            // If no paths or a shorter path found, replace the best paths
            if current.g_cost == min_cost {
                shortest_paths.push(current.path);
            }
            continue;
        }

        // Explore neighbors here...

        if current.g_cost <= max_score {
            let n = get_neighbors_2(
                input,
                current.position,
                &current.path,
                &current.g_cost,
                &mut seen,
            );
            for item in n.iter() {
                let target_direction = (
                    item.0 as isize - current.position.0 as isize,
                    item.1 as isize - current.position.1 as isize,
                );
                let mut g_cost = current.g_cost + 1;
                if target_direction.0 != current.rotation.0
                    || target_direction.1 != current.rotation.1
                {
                    g_cost += 1000;
                }

                let mut new_path = current.path.clone();

                seen.insert(*item, current.g_cost);

                new_path.insert(*item);
                open_list.push(Node {
                    position: *item,
                    rotation: target_direction,
                    g_cost,
                    path: new_path,
                });
                //path.insert(*item, current.position);
            }
        }
    }

    let mut unique_tuples: HashSet<_> = HashSet::new();
    for list in shortest_paths.iter() {
        for item in list.iter() {
            unique_tuples.insert(item);
        }
    }
    println!("Part two: {:?}", unique_tuples.len());

    return output_cost;
}

fn part_one(input: &Vec<Vec<char>>) -> usize {
    let start_pos = get_char_pos(input, &'S');
    let end_pos = get_char_pos(input, &'E');

    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    // key: where you are
    // value: where you came from
    let mut path: HashSet<(isize, isize)> = HashSet::new();

    // Push the starting node
    let start = Node {
        position: start_pos,
        rotation: (0, 1),
        g_cost: 0,
        path: HashSet::from_iter(vec![start_pos]),
    };

    open_list.push(start);
    let mut output_cost = 0;

    let mut shortest_paths = Vec::new();
    let mut min_cost = usize::MAX;

    while let Some(current) = open_list.pop() {
        if current.g_cost > min_cost {
            break;
        }

        if current.position == end_pos {
            output_cost = current.g_cost;

            if output_cost < min_cost {
                min_cost = output_cost;
                shortest_paths.clear();
            }
            // If no paths or a shorter path found, replace the best paths
            if output_cost == min_cost {
                shortest_paths.push(current.path);
            }
        }

        // Explore neighbors here...
        let n = get_neighbors(input, current.position, &path);

        for item in n.iter() {
            let target_direction = (
                item.0 as isize - current.position.0 as isize,
                item.1 as isize - current.position.1 as isize,
            );
            let mut g_cost = current.g_cost + 1;
            if target_direction.0 != current.rotation.0 || target_direction.1 != current.rotation.1
            {
                g_cost += 1000;
            }

            path.insert(*item);
            let next = Node {
                position: *item,
                rotation: target_direction,
                g_cost,
                path: path.clone(),
            };
            open_list.push(next);
            //path.insert(*item, current.position);
        }
    }

    println!("Part one: {:?}", output_cost);

    return output_cost;
}
//fn part_two(input: &Vec<Vec<char>>) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_01: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_02: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    #[test]
    fn test_part_1() {
        let map1: Vec<Vec<char>> = EXAMPLE_01.lines().map(|c| c.chars().collect()).collect();
        let map2: Vec<Vec<char>> = EXAMPLE_02.lines().map(|c| c.chars().collect()).collect();
        assert_eq!(part_one(&map1), 7036);
        assert_eq!(part_one(&map2), 11048);
    }
}
