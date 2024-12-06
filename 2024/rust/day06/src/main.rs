use core::panic;
use std::{collections::HashSet, time::Instant, usize};

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");

    let guard_map: Vec<Vec<&str>> = input
        .split_whitespace()
        .map(|c| c.split("").filter(|&cc| !cc.is_empty()).collect())
        .collect();

    // assume square map

    part_one(start, &guard_map);
    part_two(start, guard_map);
}

fn part_two(start: Instant, mut guard_map: Vec<Vec<&str>>) {
    // assume square map
    let width = guard_map.len();
    let height = guard_map.len();

    // add start position
    // loop until done
    // for x, y
    // if ., -> #
    // run part one on map, if escape ignore
    // else, increase
    let mut loops = 0;
    for y in 0..height {
        for x in 0..width {
            // place # at this position
            if guard_map[x][y] == "." {
                guard_map[x][y] = "#";
                if is_loop(&guard_map) {
                    loops += 1;
                }
                // set back to prev square
                guard_map[x][y] = ".";
            }
        }
    }
    println!("Part 2: {:?}", loops);
    println!("Time elapsed: {:.3?}", start.elapsed());
}
fn is_loop(guard_map: &Vec<Vec<&str>>) -> bool {
    // assume square map
    let width = guard_map.len();
    let height = guard_map.len();

    // init positions
    let mut spy: i32;
    let mut spx: i32;
    let mut guard: String;

    ((spx, spy), guard) = get_start_position(&guard_map);

    // same as start pos
    let mut nextx: i32 = spx;
    let mut nexty: i32 = spy;

    // save state
    let mut visited_positions: HashSet<(i32, i32)> = Default::default();

    // add start position
    visited_positions.extend(vec![(spx, spy)]);
    let mut iterator = 0;

    //for row in guard_map.clone().iter() {
    //    println!("{:?}", row);
    //}
    // loop until done
    loop {
        // if bounds are breached, exit loop
        if iterator > 10000 {
            return true;
        }

        let movement = get_move_dir(&guard);

        nextx += movement.0;
        nexty += movement.1;
        if nextx as usize >= width || nextx < 0 || nexty as usize >= height || nexty < 0 {
            visited_positions.extend(vec![(spx, spy)]);
            return false;
        }
        let next_tile = &guard_map[nexty as usize][nextx as usize];
        match next_tile {
            // save sp(x,y) in hashset, set  sp(x,y) to next
            &"." => {
                visited_positions.extend(vec![(spx, spy)]);
                spx = nextx;
                spy = nexty;
            }
            &"#" => {
                guard = rotate_guard(&guard).to_string();
                // reset nextx, nexty
                nextx = spx;
                nexty = spy;
            }
            // rotate guard
            _ => {
                // treat as .
                visited_positions.extend(vec![(spx, spy)]);
                spx = nextx;
                spy = nexty;
            }
        }
        iterator += 1;
    }
}

fn part_one(start: Instant, guard_map: &Vec<Vec<&str>>) {
    // assume square map
    let width = guard_map.len();
    let height = guard_map.len();

    // init positions
    let mut spy: i32;
    let mut spx: i32;
    let mut guard: String;

    ((spx, spy), guard) = get_start_position(&guard_map);

    // same as start pos
    let mut nextx: i32 = spx;
    let mut nexty: i32 = spy;

    // save state
    let mut visited_positions: HashSet<(i32, i32)> = Default::default();

    // add start position
    visited_positions.extend(vec![(spx, spy)]);
    // loop until done
    loop {
        // if bounds are breached, exit loop
        let movement = get_move_dir(&guard);

        nextx += movement.0;
        nexty += movement.1;
        if nextx as usize >= width || nextx < 0 || nexty as usize >= height || nexty < 0 {
            visited_positions.extend(vec![(spx, spy)]);
            break;
        }
        let next_tile = &guard_map[nexty as usize][nextx as usize];
        match next_tile {
            // save sp(x,y) in hashset, set  sp(x,y) to next
            &"." => {
                visited_positions.extend(vec![(spx, spy)]);
                spx = nextx;
                spy = nexty;
            }
            &"#" => {
                guard = rotate_guard(&guard).to_string();
                // reset nextx, nexty
                nextx = spx;
                nexty = spy;
            }
            // rotate guard
            _ => {
                // treat as .
                visited_positions.extend(vec![(spx, spy)]);
                spx = nextx;
                spy = nexty;
            }
        }
        // move
        // remove empty strings if there are any (like the last \n might create)
        //let index_to_remove = lists_to_verify.iter().position(|x| *x == "").unwrap();
    }
    println!("Part 1: {:?}", visited_positions.len());
    println!("Time elapsed: {:.3?}", start.elapsed());
}
fn rotate_guard(guard: &str) -> &str {
    match guard {
        "^" => ">",
        ">" => "v",
        "v" => "<",
        "<" => "^",
        _ => panic!("This should never happen, panic of rotate guard"),
    }
}

fn get_move_dir(guard: &str) -> (i32, i32) {
    let m = match guard {
        "^" => (0, -1),
        ">" => (1, 0),
        "v" => (0, 1),
        "<" => (-1, 0),
        _ => panic!("This should never happen, panic of move-dir guard"),
    };
    return m;
}

// rotation is e(ast), (s)outh, (w)est, (n)orth
fn get_start_position(guard_map: &Vec<Vec<&str>>) -> ((i32, i32), String) {
    let targets = ["v", "^", ">", "<"];
    for (row, line) in guard_map.iter().enumerate() {
        if let Some((col, &symb)) = line.iter().enumerate().find(|(_, &v)| targets.contains(&v)) {
            return ((col as i32, row as i32), symb.to_string());
        }
    }
    return ((0, 0), "r".to_string());
}
