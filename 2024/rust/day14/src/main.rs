use std::{collections::HashMap, time::Instant};

use regex::Regex;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
    println!("time elapsed: {:.3?}", start.elapsed());
}

fn part_one(input: &str) {
    let width = 101;
    let height = 103;
    let steps = 100;

    let mut end_positions: Vec<(i32, i32)> = vec![];
    let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").unwrap();

    for line in input.lines() {
        let Some(caps) = re.captures(line) else {
            continue;
        };
        let p1: i32 = caps[1].parse().unwrap();
        let p2: i32 = caps[2].parse().unwrap();
        let v1: i32 = caps[3].parse().unwrap();
        let v2: i32 = caps[4].parse().unwrap();

        let n_1 = (p1 + steps * v1).rem_euclid(width);
        let n_2 = (p2 + steps * v2).rem_euclid(height);
        end_positions.push((n_1, n_2));
    }

    let mut q1: u32 = 0;
    let mut q2: u32 = 0;
    let mut q3: u32 = 0;
    let mut q4: u32 = 0;
    for pos in end_positions {
        if pos.0 < width / 2 && pos.1 < height / 2 {
            q1 += 1;
        } else if pos.0 > width / 2 && pos.1 < height / 2 {
            q2 += 1;
        } else if pos.0 < width / 2 && pos.1 > height / 2 {
            q3 += 1;
        } else if pos.0 > width / 2 && pos.1 > height / 2 {
            q4 += 1;
        }
    }

    println!("Part 1: {}", q1 * q2 * q3 * q4);
}

fn contains_row(list: &Vec<(i32, i32)>) -> bool {
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();

    // Group points by their y-coordinate (row)
    for &(x, y) in list {
        rows.entry(y).or_insert_with(Vec::new).push(x);
    }

    // Check each row for 10 consecutive points
    for (_y, x_coords) in rows {
        let mut sorted_x = x_coords.clone();
        sorted_x.sort_unstable();

        let mut consecutive_count = 1; // Start with the first point
        if sorted_x.len() < 10 {
            continue;
        }
        for i in 1..sorted_x.len() {
            if sorted_x[i] == sorted_x[i - 1] + 1 {
                consecutive_count += 1;
                if consecutive_count >= 8 {
                    return true; // Found a row with exactly 10 consecutive points
                }
            } else {
                consecutive_count = 1; // Reset count
            }
        }
    }

    false
}

fn part_two(input: &str) {
    let width = 101;
    let height = 103;

    let mut positions: Vec<(i32, i32, i32, i32)> = vec![];
    let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").unwrap();

    for line in input.lines() {
        let Some(caps) = re.captures(line) else {
            continue;
        };
        let p1: i32 = caps[1].parse().unwrap();
        let p2: i32 = caps[2].parse().unwrap();
        let v1: i32 = caps[3].parse().unwrap();
        let v2: i32 = caps[4].parse().unwrap();

        positions.push((p1, p2, v1, v2));
    }

    for i in 1.. {
        let mut curr = vec![];
        for p in positions.iter() {
            let n_1 = (p.0 + i * p.2).rem_euclid(width);
            let n_2 = (p.1 + i * p.3).rem_euclid(height);
            curr.push((n_1, n_2));
        }
        if contains_row(&curr) {
            println!("Part 2: {}", i);
            break;
        }
    }
}
