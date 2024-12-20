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
fn get_path_len(input: &Vec<Vec<char>>, max_len: usize) -> (usize, HashSet<(isize, isize)>) {
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
        if current.g_cost > min_cost || current.g_cost > max_len {
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
    let map_len = map.len();
    let max_len = map_len * map_len;
    let (path_len, _) = get_path_len(&map, max_len);
    println!("base len : {}", path_len);
    part_one(&path_len);
    println!("Time elapsed: {:.3?}", start.elapsed());
}
fn get_legal_cheats(
    map: &Vec<Vec<char>>,
    cheat_pos: (usize, usize),
) -> HashSet<((usize, usize), (usize, usize))> {
    // get all cheats with cheat_pos as start point
    let mut cheat_spots = HashSet::new();
    let w = map.len() - 1; // border
                           //let start = get_char_pos(map, &'S');

    let mut left = 'x';
    let mut can_from_left = true;
    let mut right = 'x';
    let mut can_from_right = true;
    let mut down = 'x';
    let mut can_from_down = true;
    let mut up = 'x';
    let mut can_from_up = true;

    if cheat_pos.1 > 0 {
        left = map[cheat_pos.0][cheat_pos.1 - 1];
        // check if can reach from end pos
        /*
        let mut t_m = map.clone();
        t_m[cheat_pos.0][cheat_pos.1] = '#';
        t_m[start.0 as usize][start.1 as usize] = '#';
        t_m[cheat_pos.0][cheat_pos.1 - 1] = 'S';
        let t_p = get_path_len(&t_m);
        if t_p.0 > 0 {
            can_from_left = true;
        }
        */
    }
    if cheat_pos.1 < w {
        right = map[cheat_pos.0][cheat_pos.1 + 1];
        // check if can reach from end pos
        //
        /*
        let mut t_m = map.clone();
        t_m[cheat_pos.0][cheat_pos.1] = '#';
        t_m[start.0 as usize][start.1 as usize] = '#';
        t_m[cheat_pos.0][cheat_pos.1 + 1] = 'S';
        let t_p = get_path_len(&t_m);
        if t_p.0 > 0 {
            can_from_right = true;
        }
        */
    }
    if cheat_pos.0 > 0 {
        up = map[cheat_pos.0 - 1][cheat_pos.1];
        // check if can reach from end pos
        /*
        let mut t_m = map.clone();
        t_m[cheat_pos.0][cheat_pos.1] = '#';
        t_m[start.0 as usize][start.1 as usize] = '#';
        t_m[cheat_pos.0 - 1][cheat_pos.1] = 'S';
        let t_p = get_path_len(&t_m);
        if t_p.0 > 0 {
            can_from_up = true;
        }
        */
    }
    if cheat_pos.0 < w {
        down = map[cheat_pos.0 + 1][cheat_pos.1];
        // check if can reach from end pos
        /*
        let mut t_m = map.clone();
        t_m[cheat_pos.0][cheat_pos.1] = '#';
        t_m[start.0 as usize][start.1 as usize] = '#';
        t_m[cheat_pos.0 + 1][cheat_pos.1] = 'S';
        let t_p = get_path_len(&t_m);
        if t_p.0 > 0 {
            can_from_down = true;
        }
        */
    }
    //
    // from pos to right
    // left needs to be '.'
    // if right is '#' -> right + 1 needs to be '.' or 'e'
    // else if right is '.' or 'E', valid
    if (left == '.' || left == 'S') && can_from_right {
        if right == '#' && cheat_pos.1 <= w {
            // maybe w-1
            let tmp = map[cheat_pos.0][cheat_pos.1 + 2];
            if tmp == '.' {
                cheat_spots.insert((cheat_pos, (cheat_pos.0, cheat_pos.1 + 1)));
            }
        } else if right == '.' || right == 'E' {
            cheat_spots.insert((cheat_pos, (cheat_pos.0, cheat_pos.1 + 1)));
        }
    }
    // from pos to down
    // up needs to be '.'
    // if down is '#' down + 1 needs to be '.' or e
    // else if down is '.' or 'E', valid
    if (up == '.' || up == 'S') && can_from_down {
        if down == '#' && cheat_pos.0 <= w {
            // maybe w-1
            let tmp = map[cheat_pos.0 + 2][cheat_pos.1];
            if tmp == '.' {
                cheat_spots.insert((cheat_pos, (cheat_pos.0 + 1, cheat_pos.1)));
            }
        } else if down == '.' || down == 'E' {
            cheat_spots.insert((cheat_pos, (cheat_pos.0 + 1, cheat_pos.1)));
        }
    }
    // from pos to left
    // right needs to be '.'
    // if left is '#' -> left - 1 needs to be '.' or 'e'
    // else if left is '.' or 'E', valid
    if (right == '.' || right == 'S') && can_from_left {
        if left == '#' && cheat_pos.1 > 0 {
            //
            // maybe 1
            let tmp = map[cheat_pos.0][cheat_pos.1 - 2];
            if tmp == '.' {
                cheat_spots.insert((cheat_pos, (cheat_pos.0, cheat_pos.1 - 1)));
            }
        } else if left == '.' || left == 'E' {
            cheat_spots.insert((cheat_pos, (cheat_pos.0, cheat_pos.1 - 1)));
        }
    }
    //
    //
    // from pos to up
    // down needs to be '.'
    // if up is '#' up- 1 needs to be '.' or e
    // else if up is '.' or 'E', valid
    //
    if (down == '.' || down == 'S') && can_from_up {
        if up == '#' && cheat_pos.0 > 0 {
            // maybe 1
            let tmp = map[cheat_pos.0 - 2][cheat_pos.1];
            if tmp == '.' {
                cheat_spots.insert((cheat_pos, (cheat_pos.0 - 1, cheat_pos.1)));
            }
        } else if up == '.' || up == 'E' {
            cheat_spots.insert((cheat_pos, (cheat_pos.0 - 1, cheat_pos.1)));
        }
    }
    return cheat_spots;
}

fn part_one(base_len: &usize) {
    let map = parse_input();
    let mut cheat_spots = HashSet::new();
    // loop from larger than 0, less than w
    // check for '.' (row, col), (row, col+1), '.' and
    // check for '.' (row, col), (row+1, col), '.'
    for (row_i, row) in map.iter().enumerate().skip(1) {
        for (col_i, col) in row.iter().enumerate().skip(1) {
            if *col == '#' {
                let tmp_cheats = get_legal_cheats(&map, (row_i, col_i));
                cheat_spots.extend(tmp_cheats);
            }
        }
    }
    let mut saves: HashMap<usize, usize> = HashMap::new();
    for cheat in cheat_spots.iter() {
        let mut tmp_map = map.clone();
        let p1 = cheat.0;
        let p2 = cheat.1;
        let r_1 = tmp_map[p1.0][p1.1];
        let r_2 = tmp_map[p2.0][p2.1];
        if r_1 != 'S' && r_1 != 'E' {
            tmp_map[p1.0][p1.1] = '1';
        }
        if r_2 != 'S' && r_2 != 'E' {
            tmp_map[p2.0][p2.1] = '2';
        }
        let (len, p) = get_path_len(&tmp_map, *base_len);
        let diff = base_len - len;

        // path needs to contain both cheat positions
        if p.contains(&(p1.0 as isize, p1.1 as isize))
            && p.contains(&(p2.0 as isize, p2.1 as isize))
        {
            for x in p.iter() {
                let a = tmp_map[x.0 as usize][x.1 as usize];
                if a != 'S' && a != 'E' && a != '1' && a != '2' {
                    tmp_map[x.0 as usize][x.1 as usize] = 'O';
                }
            }
            if diff >= 100 {
                *saves.entry(diff).or_insert(0) += 1;
            }
        }
    }
    let mut num_of_good_cheats = 0;
    for (_, v) in saves.into_iter() {
        num_of_good_cheats += v;
    }
    println!("Part 1: {:?}", num_of_good_cheats);
}
#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!();
}
