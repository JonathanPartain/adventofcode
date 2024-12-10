use std::{collections::HashSet, time::Instant, usize};

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let digit_map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut start_pos: Vec<(usize, usize)> = vec![];
    for (row_index, row) in digit_map.iter().enumerate() {
        for (col_index, _) in row.iter().enumerate() {
            if digit_map[row_index][col_index] == 0 {
                start_pos.push((row_index, col_index));
            }
        }
    }

    part_one(&digit_map, &start_pos);
    part_two(&digit_map, &start_pos);
    println!("duration: {:.3?}", start.elapsed());
}

fn part_one(map: &Vec<Vec<u32>>, start_positions: &Vec<(usize, usize)>) {
    let mut all_scored = 0;

    for (_, start) in start_positions.iter().enumerate() {
        let mut endpoints: Vec<(usize, usize)> = Default::default();
        walk_one(map, *start, 0, map.len(), &mut endpoints);
        // get set
        let hs: HashSet<&(usize, usize)> = HashSet::from_iter(endpoints.iter());
        all_scored += hs.len();
    }

    println!("Part 1: {:?}", all_scored);
}

fn part_two(map: &Vec<Vec<u32>>, start_positions: &Vec<(usize, usize)>) {
    let mut all_scored = 0;

    for (_, start) in start_positions.iter().enumerate() {
        let mut endpoints: Vec<(usize, usize)> = Default::default();
        walk_one(map, *start, 0, map.len(), &mut endpoints);
        // get set
        all_scored += endpoints.len();
    }

    println!("Part 2: {:?}", all_scored);
}
fn walk_one(
    map: &Vec<Vec<u32>>,
    pos: (usize, usize),
    curr_height: u32,
    bound: usize,
    path: &mut Vec<(usize, usize)>,
) {
    //-> Vec<(usize, usize)> {
    let mut next: Vec<(usize, usize)> = vec![];

    if curr_height == 9 {
        path.push(pos);
        return; //path.to_vec();
    }
    //sleep(Duration::from_secs(1));
    if pos.1 > 0 {
        let left = map[pos.0][pos.1 - 1];
        if left == curr_height + 1 {
            next.push((pos.0, pos.1 - 1));
            //return walk_one(map, (pos.0, pos.1 - 1), left, bound, path);
        }
    }
    if pos.1 + 1 < bound {
        let right = map[pos.0][pos.1 + 1];
        if right == curr_height + 1 {
            next.push((pos.0, pos.1 + 1));
            //return walk_one(map, (pos.0, pos.1 + 1), right, bound, path);
        }
    }
    if pos.0 > 0 {
        let up = map[pos.0 - 1][pos.1];
        if up == curr_height + 1 {
            next.push((pos.0 - 1, pos.1));
            //return walk_one(map, (pos.0 - 1, pos.1), up, bound, path);
        }
    }
    if pos.0 + 1 < bound {
        let down = map[pos.0 + 1][pos.1];
        if down == curr_height + 1 {
            next.push((pos.0 + 1, pos.1));
            //return walk_one(map, (pos.0 + 1, pos.1), down, bound, path);
        }
    }
    for n in next {
        walk_one(map, n, curr_height + 1, bound, path);
    }
    // Dead end
    return; //vec![];
}
