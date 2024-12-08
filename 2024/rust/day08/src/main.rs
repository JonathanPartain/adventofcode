use std::{collections::HashMap, collections::HashSet, usize};

fn main() {
    let input = include_str!("../input.txt");
    let mut char_2d: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let limit: i32 = char_2d.len().try_into().unwrap();

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

    let ans = part_one(&char_pos, &existing_chars, limit);
    for a in ans.iter() {
        char_2d[a.1][a.0] = '#';
    }
}

fn part_one(
    char_pos: &HashMap<(usize, usize), char>,
    existing_chars: &HashSet<char>,
    limit: i32,
) -> HashSet<(usize, usize)> {
    let mut antinode_pos: HashSet<(usize, usize)> = Default::default();

    for char_type in existing_chars.iter() {
        let positions: Vec<(usize, usize)> = char_pos
            .iter()
            .filter(|&(_, v)| v == char_type)
            .map(|(&k, _)| k)
            .collect();

        antinode_pos.extend(get_antinodes(&positions, &limit));
    }
    // remove those whose position also exist in char_pos
    //antinode_pos.retain(|pos| !char_pos.contains_key(pos));
    println!("Part 1: {:?}", antinode_pos.len());
    return antinode_pos;
}

fn get_antinodes(positions: &Vec<(usize, usize)>, limit: &i32) -> HashSet<(usize, usize)> {
    let mut anti_nodes: HashSet<(usize, usize)> = Default::default();
    for (i_ind, i) in positions.iter().enumerate() {
        for (j_ind, j) in positions.iter().enumerate() {
            if i_ind < j_ind {
                let x_diff = (i.0).abs_diff(j.0);
                let y_diff = (i.1).abs_diff(j.1);

                // anti-nodes
                let left_x: i32;
                let left_y: i32;

                let right_x: i32;
                let right_y: i32;

                //

                // ROW
                if i.0 < j.0 {
                    left_x = i.0 as i32 - x_diff as i32;
                    right_x = j.0 as i32 + x_diff as i32;
                    // COLUMN
                    if i.1 < j.1 {
                        left_y = i.1 as i32 - y_diff as i32;

                        right_y = j.1 as i32 + y_diff as i32;
                    } else {
                        left_y = i.1 as i32 + y_diff as i32;
                        right_y = j.1 as i32 - y_diff as i32;
                    }
                } else {
                    left_x = j.0 as i32 - x_diff as i32;
                    right_x = i.0 as i32 + x_diff as i32;
                    // COLUMN
                    if i.1 < j.1 {
                        left_y = j.1 as i32 + y_diff as i32;
                        right_y = i.1 as i32 - y_diff as i32;
                    } else {
                        right_y = i.1 as i32 + y_diff as i32;
                        left_y = j.1 as i32 - y_diff as i32;
                    }
                }
                // COLUMN
                // the two positions are (left_x, left_y) and (right_x, right_y)
                if left_x >= 0 && &left_x < limit && left_y >= 0 && &left_y < limit {
                    anti_nodes.insert((left_x.try_into().unwrap(), left_y.try_into().unwrap()));
                }
                //
                if right_x >= 0 && &right_x < limit && right_y >= 0 && &right_y < limit {
                    anti_nodes.insert((right_x.try_into().unwrap(), right_y.try_into().unwrap()));
                }
            }
        }
    }
    return anti_nodes;
}
