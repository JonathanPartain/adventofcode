use std::{collections::HashSet, process::exit, usize};

fn main() {
    let input = include_str!("../input.txt");
    let char_2d: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    part_one(&char_2d);
}

fn part_one(map: &Vec<Vec<char>>) {
    let mut visited: HashSet<(usize, usize)> = Default::default();
    let mut region_map: Vec<Vec<(usize, usize)>> = vec![];
    for (r_i, row) in map.iter().enumerate() {
        for (c_i, _col) in row.iter().enumerate() {
            if visited.contains(&(r_i, c_i)) {
                continue;
            }
            // get all neighbors of same char
            let mut neighbors: Vec<(usize, usize)> = vec![];
            let _ = get_neighbors(map, map[r_i][c_i], (r_i, c_i), map.len(), &mut neighbors);
            // add positions to visited
            for p in neighbors.iter() {
                visited.insert(*p);
            }
            // fill region map, this is used for calculating fences later
            region_map.push(neighbors);
        }
    }
    let mut sum = 0;
    for region in region_map.iter() {
        let f = calc_fences(&region);
        sum += f;
    }
    println!("Part one: {}", sum);
}

fn calc_fences(l: &Vec<(usize, usize)>) -> usize {
    // Convert to SET
    let mut fences = 0;
    let set: HashSet<(usize, usize)> = HashSet::from_iter(l.iter().cloned());
    let highest = set.iter().max_by_key(|(x, _)| x).unwrap();
    let lowest = set.iter().min_by_key(|(x, _)| x).unwrap();
    let mut c = highest.0;
    //println!("highest: {} Lowest: {}", highest.0, lowest.0);
    if highest.0 == lowest.0 {
        fences += (set.len() * 2) + 2;
    } else {
        loop {
            // highest is highest number, going bottom up
            if c == highest.0 {
                let c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                let mut tmp = 0;
                // outside range
                tmp += c_w.len();

                // check above
                for item in c_w.iter() {
                    if !set.contains(&(item.0 - 1, item.1)) {
                        tmp += 1;
                    }
                    // check right
                    if !set.contains(&(item.0, item.1 + 1)) {
                        tmp += 1;
                    }
                    // check left
                    if item.1 > 0 {
                        if !set.contains(&(item.0, item.1 - 1)) {
                            tmp += 1;
                        }
                    } else {
                        // edge
                        tmp += 1;
                    }
                }
                fences += tmp;
                //println!("Row: {}, fences: {}", c, tmp);
            } else if c == lowest.0 {
                let c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                let mut tmp = 0;
                tmp += c_w.len();
                // check above
                for item in c_w.iter() {
                    // check below
                    if !set.contains(&(item.0 + 1, item.1)) {
                        tmp += 1;
                    }
                    // check right
                    if !set.contains(&(item.0, item.1 + 1)) {
                        tmp += 1;
                    }
                    // check left
                    if item.1 > 0 {
                        if !set.contains(&(item.0, item.1 - 1)) {
                            tmp += 1;
                        }
                    } else {
                        // edge
                        tmp += 1;
                    }
                }
                fences += tmp;
                // println!("Row: {}, fences: {}", c, tmp);
                break;
            } else {
                // middle row
                // items in current row
                let mut tmp = 0;
                let c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                for item in c_w.iter() {
                    // check above
                    if !set.contains(&(item.0 - 1, item.1)) {
                        tmp += 1;
                    }
                    // check below
                    if !set.contains(&(item.0 + 1, item.1)) {
                        tmp += 1;
                    }
                    // check right
                    if !set.contains(&(item.0, item.1 + 1)) {
                        tmp += 1;
                    }
                    // check left
                    if item.1 > 0 {
                        if !set.contains(&(item.0, item.1 - 1)) {
                            tmp += 1;
                        }
                    } else {
                        // edge
                        tmp += 1;
                    }
                }
                // add to fences
                fences += tmp;
                //println!("Row: {}, fences: {}", c, tmp);
            }
            c -= 1;
        }
    }
    //println!("Area land: {}, fences: {}", set.len(), fences);
    return fences * set.len();
}

fn get_neighbors(
    map: &Vec<Vec<char>>,
    c: char,
    pos: (usize, usize),
    bound: usize,
    path: &mut Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    //-> Vec<(usize, usize)> {
    let mut next: Vec<(usize, usize)> = vec![];

    // add current pos to path
    path.push(pos);
    if pos.1 > 0 {
        let left = map[pos.0][pos.1 - 1];
        if left == c && !path.contains(&(pos.0, pos.1 - 1)) {
            next.push((pos.0, pos.1 - 1));
        }
    }
    if pos.1 + 1 < bound {
        let right = map[pos.0][pos.1 + 1];
        if right == c && !path.contains(&(pos.0, pos.1 + 1)) {
            next.push((pos.0, pos.1 + 1));
        }
    }
    if pos.0 > 0 {
        let up = map[pos.0 - 1][pos.1];
        if up == c && !path.contains(&(pos.0 - 1, pos.1)) {
            next.push((pos.0 - 1, pos.1));
        }
    }
    if pos.0 + 1 < bound {
        let down = map[pos.0 + 1][pos.1];
        if down == c && !path.contains(&(pos.0 + 1, pos.1)) {
            next.push((pos.0 + 1, pos.1));
        }
    }
    let res: Vec<(usize, usize)> = next.clone();
    for n in res.iter() {
        get_neighbors(map, c, *n, bound, path);
    }
    // Dead end
    return next; //vec![];
}
