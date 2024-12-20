use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
    usize, vec,
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
            if grid[nx][ny] == '2' {
                // make sure we have walked 1 first
                let pos_1 = get_char_pos(grid, &'1');
                if path.contains(&pos_1) {
                    neighbors.push((nx as isize, ny as isize));
                }
            } else {
                neighbors.push((nx as isize, ny as isize));
            }
        }
    }
    neighbors
}
fn get_path_len(input: &Vec<Vec<char>>, max_len: &usize) -> (usize, HashSet<(isize, isize)>) {
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    // key: where you are
    // value: where you came from
    let start_pos = get_char_pos(&input, &'S');
    let end_pos = get_char_pos(&input, &'E');

    // Push the starting node
    let start = Node {
        position: start_pos,
        g_cost: 0,
        path: HashSet::from_iter(vec![start_pos]),
    };

    open_list.push(start);
    let mut output_cost = 0;
    let mut min_cost = usize::MAX;
    let mut out_path = HashSet::new();

    while let Some(current) = open_list.pop() {
        if current.g_cost > min_cost || current.g_cost > *max_len {
            break;
        }

        if current.position == end_pos {
            output_cost = current.g_cost;
            if output_cost < min_cost {
                min_cost = output_cost
            }
            if output_cost == min_cost {
                out_path = current.path.clone();
            }
        }

        // Explore neighbors here...
        let n = get_neighbors(input, current.position, &current.path);

        for item in n.iter() {
            let g_cost = current.g_cost + 1;

            let mut new_path = current.path.clone();
            new_path.insert(*item);
            let next = Node {
                position: *item,
                g_cost,
                path: new_path,
            };
            open_list.push(next);
            //path.insert(*item, current.position);
        }
    }

    return (output_cost, out_path);
}

fn parse_input() -> Vec<Vec<char>> {
    let input: &str = include_str!("../input.txt");
    let map: Vec<Vec<char>> = input.lines().map(|c| c.chars().collect()).collect();
    return map;
}

fn main() {
    let start = Instant::now();
    let map = parse_input();

    let min_diff = 100;

    let path_cost_to_end = get_all_positions(&map);

    let mut s_modified = map.clone();
    let s_m_e = get_char_pos(&s_modified, &'E');
    let s_m_s = get_char_pos(&s_modified, &'S');
    s_modified[s_m_e.0 as usize][s_m_e.1 as usize] = '.';
    s_modified[s_m_s.0 as usize][s_m_s.1 as usize] = 'E';

    let path_cost_from_start = get_all_positions(&s_modified);

    let p1 = solve(
        &map,
        &path_cost_to_end,
        &path_cost_from_start,
        2,
        min_diff,
        &usize::MAX,
    );
    println!("Part 1: {}", p1);
    let p2 = solve(
        &map,
        &path_cost_to_end,
        &path_cost_from_start,
        20,
        min_diff,
        &usize::MAX,
    );
    println!("Part 2: {}", p2);
    println!("Time elapsed: {:.3?}", start.elapsed());
}

fn solve(
    input: &Vec<Vec<char>>,
    path_cost_to_end: &HashMap<(usize, usize), usize>,
    path_cost_from_start: &HashMap<(usize, usize), usize>,
    mh_val: usize,
    min_diff: usize,
    max_len: &usize,
) -> usize {
    // base path
    let (base_len, path) = get_path_len(input, max_len);

    // hashmap containing start and end points of cheats
    let mut cheats: HashSet<((usize, usize), (usize, usize))> = HashSet::default();

    for pos in path.iter() {
        let p_x = pos.0;
        let p_y = pos.1;

        let legal_starts = get_cheat_start_positions(input, &(p_x as usize, p_y as usize));

        let pos_in_dis = positions_within_distance(
            p_x as usize,
            p_y as usize,
            mh_val,
            input.len(),
            input[0].len(),
        );

        for _ in legal_starts.iter() {
            // legal neighors, path from start to current pos in path +1
            let current_val = path_cost_from_start
                .get(&(pos.0 as usize, pos.1 as usize))
                .unwrap()
                + 1;

            // for all end_positions
            for end_pos in pos_in_dis.iter() {
                // get value from end position to the end
                if let Some(target_val) = path_cost_to_end.get(end_pos) {
                    // manhattan distance of "cheat" path
                    // abs(x_1 - x_2) + abs(y_1-y_2)
                    let cheat_time = (p_x as isize - end_pos.0 as isize).abs()
                        + (p_y as isize - end_pos.1 as isize).abs();

                    // path to legal_start position + len of cheat_time + from end_position to end
                    // of path + min_diff (100) needs to be less than base_len to be worth
                    if current_val + cheat_time as usize + target_val + min_diff < base_len {
                        cheats.insert((
                            (p_x as usize, p_y as usize),
                            (end_pos.0 as usize, end_pos.1 as usize),
                        ));
                    }
                }
            }
        }
    }
    return cheats.len();
}

fn get_cheat_start_positions(
    input: &Vec<Vec<char>>,
    curr_pos: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut adjacents = Vec::new();
    let c_x = curr_pos.0;
    let c_y = curr_pos.1;

    let neighbors = [
        (c_x.wrapping_sub(1), c_y), // left
        (c_x + 1, c_y),             // right
        (c_x, c_y.wrapping_sub(1)), // up
        (c_x, c_y + 1),             // down
    ];

    for &(nx, ny) in &neighbors {
        // starts can only happen in walls
        if nx < input[c_y].len() && ny < input.len() && input[ny][nx] == '#' {
            adjacents.push((nx, ny)); // valid adjacent to track
        }
    }

    adjacents
}

fn positions_within_distance(
    x: usize,
    y: usize,
    d: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for dx in 0..=d {
        let dy_max = d - dx;
        for dy in 0..=dy_max {
            let new_positions = [
                (x + dx, y + dy),                             // Top-right quadrant
                (x + dx, y.saturating_sub(dy)),               // Bottom-right quadrant
                (x.saturating_sub(dx), y + dy),               // Top-left quadrant
                (x.saturating_sub(dx), y.saturating_sub(dy)), // Bottom-left quadrant
            ];

            for &(nx, ny) in &new_positions {
                if nx <= max_x && ny <= max_y && !positions.contains(&(nx, ny)) {
                    positions.push((nx, ny));
                }
            }
        }
    }

    positions
}

// get values for ALL positions to the "end"
fn get_all_positions(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), usize> {
    let e_pos = get_char_pos(map, &'E');

    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();

    let mut all_pos_costs: HashMap<(usize, usize), usize> = HashMap::new();
    // Push the starting node
    let start = Node {
        position: e_pos,
        g_cost: 0,
        path: HashSet::from_iter(vec![e_pos]),
    };

    open_list.push(start);
    all_pos_costs.insert((e_pos.0 as usize, e_pos.1 as usize), 1);

    while let Some(current) = open_list.pop() {
        // Explore neighbors here...
        let n = get_neighbors(map, current.position, &current.path);

        for item in n.iter() {
            let g_cost = current.g_cost + 1;

            all_pos_costs.insert((item.0 as usize, item.1 as usize), current.g_cost);

            let mut new_path = current.path.clone();
            new_path.insert(*item);
            let next = Node {
                position: *item,
                g_cost,
                path: new_path,
            };
            open_list.push(next);
            //path.insert(*item, current.position);
        }
    }

    return all_pos_costs;
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!();
}
