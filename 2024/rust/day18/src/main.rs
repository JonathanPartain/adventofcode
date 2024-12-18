use std::{
    collections::{BinaryHeap, HashSet},
    isize,
    time::Instant,
    usize,
};

#[derive(Eq, PartialEq)]
// yoinked from mr gippity
#[derive(Debug)]
struct Node {
    position: (isize, isize),
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

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!();
}

fn main() {
    // build map
    let start = Instant::now();
    let w = 71;
    let bytes_allowed = 1024;
    let input: &str = include_str!("../input.txt");
    let mut map: Vec<Vec<char>> = vec![vec!['.'; w]; w];

    let coords: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|l| {
            let mut numbers = l.split(",");
            if let (Some(a), Some(b)) = (numbers.next(), numbers.next()) {
                if let (Ok(a), Ok(b)) = (a.trim().parse(), b.trim().parse()) {
                    return Some((a, b));
                }
            }
            None
        })
        .collect();
    let mut bytes_dropped = 0;
    for (row, col) in coords.iter() {
        if bytes_dropped == bytes_allowed {
            break;
        }
        map[*col][*row] = '#';
        bytes_dropped += 1;
    }
    let (part_one, _) = part_one(&map);
    println!("Part one: {:?}", part_one);
    let part_two = part_two(&coords, w);
    println!("Part two: {:?}", part_two);
    println!("Time elapsed: {:?}", start.elapsed());
}

fn part_two(coords: &Vec<(usize, usize)>, w: usize) -> (usize, usize) {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; w]; w];
    let (mut path, mut vec_paths) = part_one(&map);
    let mut combined: HashSet<(isize, isize)> = vec_paths.into_iter().flat_map(|set| set).collect();
    for (_, (row, col)) in coords.iter().enumerate() {
        map[*row][*col] = '#';
        if combined.contains(&(*row as isize, *col as isize)) {
            // coordinate we are placing exists in path
            (path, vec_paths) = part_one(&map);
            combined = vec_paths.into_iter().flat_map(|set| set).collect();
        } else {
            continue;
        }
        if path == 0 {
            return (*row, *col);
        }
    }
    unreachable!();
}

fn part_one(input: &Vec<Vec<char>>) -> (usize, Vec<HashSet<(isize, isize)>>) {
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    // key: where you are
    // value: where you came from
    let mut path: HashSet<(isize, isize)> = HashSet::new();
    let start_pos = (0, 0);
    let end_pos = (input.len() as isize - 1, input.len() as isize - 1);

    // Push the starting node
    let start = Node {
        position: start_pos,
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
            let g_cost = current.g_cost + 1;

            path.insert(*item);
            let next = Node {
                position: *item,
                g_cost,
                path: path.clone(),
            };
            open_list.push(next);
            //path.insert(*item, current.position);
        }
    }

    return (output_cost, shortest_paths);
}
