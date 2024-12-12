use std::{collections::HashSet, process::exit, usize};

fn main() {
    let input = include_str!("../medium.txt");
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
    let mut part2 = 0;
    for region in region_map.iter() {
        //println!("region: {:?}", region);
        let f = calc_fences(&region);
        let _sides = calc_edges(&region);
        let test: Vec<_> = region.into_iter().map(|&(a, b)| (b, a)).collect();
        let set: HashSet<(usize, usize)> = HashSet::from_iter(region.iter().cloned());
        println!("{:?}", test);
        println!();
        let inv = calc_edges(&test);
        //println!("sides: {:?}", inv);
        //println!("sum: {:?}", _sides + inv);
        println!();
        println!(
            "({} + {}) * {} = {}",
            _sides,
            inv,
            set.len(),
            (_sides + inv) * set.len()
        );
        println!();
        println!();
        part2 += (_sides + inv) * set.len();

        sum += f;
    }
    println!("Part one: {}", sum);

    println!("Part two: {}", part2);
    println!("{:?}", part2 == 885394);
}

fn calc_edges(l: &Vec<(usize, usize)>) -> usize {
    // Convert to SET
    let set: HashSet<(usize, usize)> = HashSet::from_iter(l.iter().cloned());
    let highest = set.iter().max_by_key(|(x, _)| x).unwrap();
    let lowest = set.iter().min_by_key(|(x, _)| x).unwrap();
    let mut sides = 0;
    let mut c = highest.0;
    if highest.0 == lowest.0 || highest.1 == lowest.1 {
        sides = 2;
    } else {
        loop {
            let mut tmp = 0;
            // highest is highest number, going bottom up
            if c == highest.0 {
                let mut c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                c_w.sort_by(|x, y| x.1.cmp(&y.1));

                println!("set: {:?} ", c_w);

                sides += 1; // outside range
                tmp += 1; // outside range
                let mut start_side = false;

                for item in c_w.iter() {
                    if item.1 > 0 {
                        // check if previous number was in
                        if !set.contains(&(item.0, item.1 - 1)) {
                            // prev item is not directly to the left
                            if start_side {
                                sides += 1;
                                tmp += 1;
                            }
                        }
                    }
                    // check above
                    if !set.contains(&(item.0 - 1, item.1)) {
                        println!("highest: Found side start above {:?}", item);
                        start_side = true;
                    } else if start_side {
                        // side has started and ended
                        println!("highest: start above ended {:?}", item);
                        sides += 1;
                        tmp += 1;
                        start_side = false;
                    }
                }
                if start_side {
                    println!("highest: loop ended with start side true ");
                    sides += 1; // loop ended with start_side true
                    tmp += 1;
                }
                //println!("Row: {}, fences: {}", c, tmp);
            } else if c == lowest.0 {
                let mut c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                c_w.sort_by(|x, y| x.1.cmp(&y.1));
                println!("set: {:?} ", c_w);

                sides += 1; // outside range
                tmp += 1; // outside range
                let mut start_side = false;

                for item in c_w.iter() {
                    if item.1 > 0 {
                        // check if previous number was in
                        if !set.contains(&(item.0, item.1 - 1)) {
                            // prev item is not directly to the left
                            if start_side {
                                sides += 1;
                                tmp += 1;
                            }
                        }
                    }
                    // check above
                    if !set.contains(&(item.0 + 1, item.1)) {
                        //    println!("lowest: Found side start below {:?}", item);
                        start_side = true;
                    } else if start_side {
                        // side has started and ended
                        println!("lowest: side start ended {:?}", item);
                        sides += 1;
                        tmp += 1;
                        start_side = false;
                    }
                }
                if start_side {
                    println!("lowest: loop ended with start side true ");
                    sides += 1; // loop ended with start_side true
                    tmp += 1;
                }
                // println!("Row: {}, fences: {}", c, tmp);
                println!("sides for this set: {}", tmp);
                break;
            } else {
                // middle row
                // items in current row
                let mut c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                c_w.sort_by(|x, y| x.1.cmp(&y.1));
                println!("set: {:?} ", c_w);
                let mut start_below = false;
                let mut start_above = false;
                for item in c_w.iter() {
                    if item.1 > 0 {
                        // check if previous number was in
                        if !set.contains(&(item.0, item.1 - 1)) {
                            // prev item is not directly to the left
                            if start_below {
                                sides += 1;
                                tmp += 1;
                            }
                            if start_above {
                                sides += 1;
                                tmp += 1;
                            }
                        }
                    }
                    // check above
                    if !set.contains(&(item.0 - 1, item.1)) {
                        println!("middle: side start above {:?}", item);
                        start_above = true;
                    } else if start_above {
                        // side has started and ended
                        println!("middle: side start above ended {:?}", item);
                        sides += 1;
                        tmp += 1;
                        start_above = false;
                    }
                    if !set.contains(&(item.0 + 1, item.1)) {
                        println!("middle: side start below {:?}", item);
                        start_below = true;
                    } else if start_below {
                        println!("middle: side start below ended {:?}", item);
                        // side has started and ended
                        sides += 1;
                        tmp += 1;
                        start_below = false;
                    }
                    // no item
                }
                if start_below {
                    println!("middle: loop ended with start below");
                    sides += 1; // loop ended with start_side true
                    tmp += 1;
                }
                if start_above {
                    println!("middle: loop ended with start above");
                    sides += 1; // loop ended with start_side true
                    tmp += 1;
                }
            }
            c -= 1;

            println!("sides for this set: {}", tmp);
        }
    }
    return sides;
}
fn calc_fences(l: &Vec<(usize, usize)>) -> usize {
    // Convert to SET
    let mut fences = 0;
    let set: HashSet<(usize, usize)> = HashSet::from_iter(l.iter().cloned());
    let highest = set.iter().max_by_key(|(x, _)| x).unwrap();
    let lowest = set.iter().min_by_key(|(x, _)| x).unwrap();
    let mut _sides = 0;
    let mut c = highest.0;
    //println!("highest: {} Lowest: {}", highest.0, lowest.0);
    if highest.0 == lowest.0 {
        fences += (set.len() * 2) + 2;
        _sides = 4;
    } else {
        loop {
            // highest is highest number, going bottom up
            if c == highest.0 {
                let c_w: Vec<&(usize, usize)> = set.iter().filter(|(x, _)| *x == c).collect();
                let mut tmp = 0;
                // outside range
                tmp += c_w.len();

                for item in c_w.iter() {
                    // check above
                    if !set.contains(&(item.0 - 1, item.1)) {
                        tmp += 1;
                    }
                    // check right
                    if !set.contains(&(item.0, item.1 + 1)) {
                        tmp += 1;
                    } // check left
                    if item.1 > 0 {
                        if !set.contains(&(item.0, item.1 - 1)) {
                            tmp += 1;
                        } else {
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
