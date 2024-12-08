use std::{collections::HashMap, collections::HashSet, usize};

fn main() {
    let input = include_str!("../small.txt");
    let char_2d: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let limit = char_2d.len();
    println!("{:?}", char_2d);

    let mut char_pos: HashMap<(usize, usize), char> = Default::default();
    let mut existing_chars: HashSet<char> = Default::default();

    for (row_num, row) in char_2d.iter().enumerate() {
        for (col_num, &col) in row.iter().enumerate() {
            if col != '.' {
                existing_chars.insert(col);
                char_pos.insert((col_num, row_num), col);
            }
        }
    }
    println!("{:?}", char_pos);
    part_one(&char_pos, &existing_chars, limit);
}

fn part_one(
    char_pos: &HashMap<(usize, usize), char>,
    existing_chars: &HashSet<char>,
    limit: usize,
) {
    let mut antinode_pos: HashSet<(usize, usize)> = Default::default();

    for char_type in existing_chars.iter() {
        let positions: Vec<(usize, usize)> = char_pos
            .iter()
            .filter(|&(_, v)| v == char_type)
            .map(|(&k, _)| k)
            .collect();

        println!("{:?} positions: {:?}", char_type, positions);
        antinode_pos.extend(get_antinodes(&positions, &limit));
    }
}

fn get_antinodes(positions: &Vec<(usize, usize)>, limit: &usize) -> HashSet<(usize, usize)> {
    let mut anti_nodes: HashSet<(usize, usize)> = Default::default();
    for (i_ind, i) in positions.iter().enumerate() {
        for (j_ind, j) in positions.iter().enumerate() {
            if i_ind != j_ind {
                let x = (i.0).abs_diff(j.0);
                let y = (i.1).abs_diff(j.1);
                // get 2 possible places
                let p10 = j.0.checked_sub(x);
                let p11 = j.1.checked_sub(y);
                if p10.is_some() && p11.is_some() {
                    let p1 = (p10.unwrap(), p11.unwrap());
                    if !positions.contains(&p1)
                        && p1.0 > 0
                        && &p1.0 < limit
                        && p1.1 > 0
                        && &p1.1 < limit
                    {
                        anti_nodes.insert(p1);
                    }
                }

                let p2: (usize, usize) = (j.0 + x, j.1 + y);
                if !positions.contains(&p2)
                    && p2.0 > 0
                    && &p2.0 < limit
                    && p2.1 > 0
                    && &p2.1 < limit
                {
                    anti_nodes.insert(p2);
                }
            }
        }
    }
    println!("antinodes: {:?}, len: {:?}", anti_nodes, anti_nodes.len());
    return anti_nodes;
}
