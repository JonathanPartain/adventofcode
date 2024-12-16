use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    isize, usize,
};
#[derive(Eq, PartialEq)]

// yoinked from mr gippity
struct Node {
    position: (isize, isize),
    rotation: (isize, isize), // [(0, -1), (1, 0), (0, 1), (-1, 0)]; // North, East, South, West
    g_cost: usize,
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
    let input = parse_input();
    part_one(&input);
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
    path: &HashMap<(isize, isize), (isize, isize)>,
) -> Vec<(isize, isize)> {
    let (x, y) = position;
    let mut neighbors = Vec::new();

    // Possible moves: up, down, left, right
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in deltas.iter() {
        let nx = x.wrapping_add(*dx as isize) as usize;

        let ny = y.wrapping_add(*dy as isize) as usize;

        if nx < grid.len()
            && ny < grid[0].len()
            && grid[nx][ny] != '#'
            && !path.contains_key(&(nx as isize, ny as isize))
        {
            neighbors.push((nx as isize, ny as isize));
        }
    }
    neighbors
}

// Step function:
// map - map we are walking through
// visited - places we have been
// end_pos - final pos
// prev_pos - where I was
// rotation - what rotation do I have
// current_score - need to know which is cheaper

fn part_one(input: &Vec<Vec<char>>) -> usize {
    let start_pos = get_char_pos(input, &'S');
    let end_pos = get_char_pos(input, &'E');
    for row in input.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!();

    let mut open_list = BinaryHeap::new();
    // key: where you are
    // value: where you came from
    let mut path: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut g_costs: HashMap<(isize, isize), usize> = HashMap::new();

    // Push the starting node
    open_list.push(Node {
        position: start_pos,
        rotation: (0, 1),
        g_cost: 0,
    });
    let mut output_cost = 0;

    // A* loop (to be implemented)
    while let Some(current) = open_list.pop() {
        if current.position == end_pos {
            output_cost = current.g_cost;
            break;
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
                println!("position: {:?}", current.position);
                println!("Target dir: {:?}", target_direction);
                println!("current rot: {:?}", current.rotation);
                g_cost += 1000;
            }
            if g_cost < *g_costs.get(item).unwrap_or(&usize::MAX) {
                g_costs.insert(*item, g_cost);
                open_list.push(Node {
                    position: *item,
                    rotation: target_direction,
                    g_cost,
                });
                path.insert(*item, current.position);
            }

            println!("current cost: {}", g_cost);
            open_list.push(Node {
                position: *item,
                rotation: target_direction,
                g_cost,
            });
            path.insert(*item, current.position);
        }
    }
    let mut path_vec = vec![end_pos];
    let mut current = end_pos;

    println!("{:?}", path_vec);
    println!("{:?}", output_cost);

    return output_cost;
}

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
