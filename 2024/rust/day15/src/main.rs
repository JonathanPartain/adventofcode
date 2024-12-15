use std::{isize, time::Instant, usize};

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut map: Vec<Vec<char>> = sections[0].lines().map(|c| c.chars().collect()).collect();
    let instructions: Vec<char> = sections[1].chars().filter(|&c| c != '\n').collect();
    part_one(&mut map, &instructions);
    part_two(&mut map, &instructions);
    println!("time elapsed: {:.3?}", start.elapsed());
}

fn part_two(map: &Vec<Vec<char>>, instr: &Vec<char>) {
    let mut newmap: Vec<Vec<char>> = vec![vec![]; map.len()];
    for (row_i, row) in map.iter().enumerate() {
        newmap[row_i] = vec![];
        for (_, col) in row.iter().enumerate() {
            match col {
                '#' => {
                    newmap[row_i].push('#');
                    newmap[row_i].push('#');
                }
                'O' => {
                    newmap[row_i].push('[');
                    newmap[row_i].push(']');
                }
                '.' => {
                    newmap[row_i].push('.');
                    newmap[row_i].push('.');
                }
                '@' => {
                    newmap[row_i].push('@');
                    newmap[row_i].push('.');
                }
                _ => unreachable!(),
            }
        }
    }
    let (mut curr_row, mut curr_col) = get_start_position(&newmap);
    for inst in instr.iter() {
        /*
        * useful for stepping for debug
                let mut ans = String::new();
                let _ = io::stdin().read_line(&mut ans);
        */
        match inst {
            '^' => {
                (curr_row, curr_col) = wide_up(&mut newmap, &mut (curr_row, curr_col), -1);
            }
            '>' => {
                (curr_row, curr_col) = move_p2(&mut newmap, &mut (curr_row, curr_col), 1);
            }
            'v' => {
                (curr_row, curr_col) = wide_up(&mut newmap, &mut (curr_row, curr_col), 1);
            }
            '<' => {
                (curr_row, curr_col) = move_p2(&mut newmap, &mut (curr_row, curr_col), -1);
            }
            _ => panic!("Invalid instruction!"),
        }
    }
    let score = calculate_gps(&newmap);
    println!("Part 2: {}", score);
}
fn check_above(
    map: &mut Vec<Vec<char>>,
    from_pos: &(usize, usize),
    dir: isize,
) -> (bool, Vec<(char, usize, usize)>) {
    let next_row = from_pos.0 as isize + dir;
    let next_tile = map[next_row as usize][from_pos.1];
    let mut positions = vec![];
    match next_tile {
        '.' => return (true, positions),
        '#' => return (false, positions),
        '[' => {
            positions.push(('[', next_row as usize, from_pos.1));
            positions.push((']', next_row as usize, from_pos.1 + 1));
            let (result1, mut positions1) = check_above(map, &(next_row as usize, from_pos.1), dir);
            let (result2, mut positions2) =
                check_above(map, &(next_row as usize, from_pos.1 + 1), dir);

            positions.append(&mut positions1);
            positions.append(&mut positions2);
            (result1 && result2, positions)
        }
        ']' => {
            positions.push((']', next_row as usize, from_pos.1));
            positions.push(('[', next_row as usize, from_pos.1 - 1));
            let (result1, mut positions1) = check_above(map, &(next_row as usize, from_pos.1), dir);
            let (result2, mut positions2) =
                check_above(map, &(next_row as usize, from_pos.1 - 1), dir);
            positions.append(&mut positions1);
            positions.append(&mut positions2);
            (result1 && result2, positions)
        }
        _ => unreachable!(),
    }
}
fn wide_up(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), dir: isize) -> (usize, usize) {
    let next_row = pos.0 as isize + dir;

    let next = map[next_row as usize][pos.1];
    if next == '.' {
        map[next_row as usize][pos.1] = '@';
        map[pos.0][pos.1] = '.';
        pos.0 = next_row as usize;
    } else if next == '#' {
        // ignore
        return (pos.0, pos.1);
    } else if next == '[' {
        let (res_l, pos_l) = check_above(map, pos, dir);
        let (res_r, pos_r) = check_above(map, &(pos.0, pos.1 + 1), dir);
        if res_l && res_r {
            for item in pos_l.iter() {
                map[item.1][item.2] = '.';
            }
            for item in pos_r.iter() {
                map[item.1][item.2] = '.';
            }
            for item in pos_l.iter() {
                let tmp = item.1 as isize + dir;
                map[tmp as usize][item.2] = item.0;
                map[tmp as usize][item.2] = item.0;
            }
            for item in pos_r.iter() {
                let tmp = item.1 as isize + dir;
                map[tmp as usize][item.2] = item.0;
                map[tmp as usize][item.2] = item.0;
            }
            map[next_row as usize][pos.1] = '@';
            map[pos.0][pos.1] = '.';
            pos.0 = next_row as usize;
        } // check
    } else if next == ']' {
        let (res_l, pos_l) = check_above(map, pos, dir);
        let (res_r, pos_r) = check_above(map, &(pos.0, pos.1 - 1), dir);
        if res_l && res_r {
            for item in pos_l.iter() {
                map[item.1][item.2] = '.';
            }
            for item in pos_r.iter() {
                map[item.1][item.2] = '.';
            }
            for item in pos_l.iter() {
                let tmp = item.1 as isize + dir;
                map[tmp as usize][item.2] = item.0;
                map[tmp as usize][item.2] = item.0;
            }
            for item in pos_r.iter() {
                let tmp = item.1 as isize + dir;
                map[tmp as usize][item.2] = item.0;
                map[tmp as usize][item.2] = item.0;
            }
            map[next_row as usize][pos.1] = '@';
            map[pos.0][pos.1] = '.';
            pos.0 = next_row as usize;
        } // check
    }
    return (pos.0, pos.1);
}
// left or right only
fn move_p2(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), dir: isize) -> (usize, usize) {
    // right is row , col +1
    let mut next_col = pos.1 as isize + dir;

    // Ensure the next position is valid
    let next = map[pos.0][next_col as usize];
    if next == '.' {
        map[pos.0][next_col as usize] = '@';
        map[pos.0][pos.1] = '.';
        pos.1 = next_col as usize;
    } else if next == '#' {
        // ignore
        return (pos.0, pos.1);
    } else if next == '[' || next == ']' {
        // collect all items until . or #
        let mut items = vec![];
        loop {
            let tmp_char = map[pos.0][next_col as usize];
            match tmp_char {
                '#' => {
                    items = vec![];
                    break;
                }
                '.' => break,
                _ => items.push(tmp_char),
            }
            next_col += dir;
        }

        for (ind, item) in items.iter().enumerate() {
            let offset = (ind as isize + 2) * dir;
            let next = pos.1 as isize + offset;
            map[pos.0][next as usize] = *item;
        }

        if items.len() > 0 {
            let tmp = pos.1 as isize + dir;
            map[pos.0][tmp as usize] = '@';
            map[pos.0][pos.1] = '.';
            pos.1 = tmp as usize;
        }
    }
    return (pos.0, pos.1);
}

fn part_one(map: &mut Vec<Vec<char>>, instr: &Vec<char>) {
    let (mut curr_row, mut curr_col) = get_start_position(map);
    for inst in instr.iter() {
        match inst {
            '^' => {
                (curr_row, curr_col) = move_up(map, &mut (curr_row, curr_col));
            }
            '>' => {
                (curr_row, curr_col) = move_right(map, &mut (curr_row, curr_col));
            }
            'v' => {
                (curr_row, curr_col) = move_down(map, &mut (curr_row, curr_col));
            }
            '<' => {
                (curr_row, curr_col) = move_left(map, &mut (curr_row, curr_col));
            }
            _ => panic!("Invalid instruction!"),
        }
        //print_map(map);
    }
    let score = calculate_gps(map);
    println!("Part 1: {}", score);
}
fn calculate_gps(map: &Vec<Vec<char>>) -> usize {
    let mut gps_calc = 0;

    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == 'O' {
                gps_calc += 100 * row_i + col_i;
            }
            if *col == '[' {
                gps_calc += 100 * row_i + col_i;
            }
        }
    }
    return gps_calc;
}

fn move_up(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) -> (usize, usize) {
    // up is row -1 , col
    let above = map[pos.0 - 1][pos.1];
    if above == '.' {
        map[pos.0 - 1][pos.1] = '@';
        map[pos.0][pos.1] = '.';
        pos.0 = pos.0 - 1;
    } else if above == '#' {
        // ignore
    } else if above == 'O' {
        // collect all items until . or #
        let mut items = vec![];
        items.push('O');
        let mut tmp_pos = pos.0 - 1; // above variable == tmp_pos
        loop {
            tmp_pos -= 1;
            let tmp_char = map[tmp_pos][pos.1];
            match tmp_char {
                '#' => {
                    items = vec![];
                    break;
                }
                '.' => break,
                'O' => items.push('O'),
                _ => panic!("alarm move up"),
            }
        }
        for (ind, item) in items.iter().enumerate() {
            map[pos.0 - (ind + 2)][pos.1] = *item;
        }
        if items.len() > 0 {
            map[pos.0 - 1][pos.1] = '@';
            map[pos.0][pos.1] = '.';
            pos.0 = pos.0 - 1;
        }
    }
    return (pos.0, pos.1);
}
fn move_right(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) -> (usize, usize) {
    // right is row , col +1
    let right = map[pos.0][pos.1 + 1];
    if right == '.' {
        map[pos.0][pos.1 + 1] = '@';
        map[pos.0][pos.1] = '.';
        pos.1 = pos.1 + 1;
    } else if right == '#' {
        // ignore
    } else if right == 'O' {
        // collect all items until . or #
        let mut items = vec![];
        items.push('O');
        let mut tmp_pos = pos.1 + 1; // above variable == tmp_pos
        loop {
            tmp_pos += 1;
            let tmp_char = map[pos.0][tmp_pos];
            match tmp_char {
                '#' => {
                    items = vec![];
                    break;
                }
                '.' => break,
                'O' => items.push('O'),
                _ => panic!("alarm move up"),
            }
        }
        for (ind, item) in items.iter().enumerate() {
            map[pos.0][pos.1 + (ind + 2)] = *item;
        }
        if items.len() > 0 {
            map[pos.0][pos.1 + 1] = '@';
            map[pos.0][pos.1] = '.';
            pos.1 = pos.1 + 1;
        }
    }
    return (pos.0, pos.1);
}
fn move_down(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) -> (usize, usize) {
    // down is row +1, col
    let down = map[pos.0 + 1][pos.1];
    if down == '.' {
        map[pos.0 + 1][pos.1] = '@';
        map[pos.0][pos.1] = '.';
        pos.0 = pos.0 + 1;
    } else if down == '#' {
        // ignore
    } else if down == 'O' {
        // collect all items until . or #
        let mut items = vec![];
        items.push('O');
        let mut tmp_pos = pos.0 + 1; // above variable == tmp_pos
        loop {
            tmp_pos += 1;
            let tmp_char = map[tmp_pos][pos.1];
            match tmp_char {
                '#' => {
                    items = vec![];
                    break;
                }
                '.' => break,
                'O' => items.push('O'),
                _ => panic!("alarm move up"),
            }
        }
        for (ind, item) in items.iter().enumerate() {
            map[pos.0 + (ind + 2)][pos.1] = *item;
        }
        if items.len() > 0 {
            map[pos.0 + 1][pos.1] = '@';
            map[pos.0][pos.1] = '.';
            pos.0 = pos.0 + 1;
        }
    }
    return (pos.0, pos.1);
}
fn move_left(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize)) -> (usize, usize) {
    // left is row , col -1
    let left = map[pos.0][pos.1 - 1];
    if left == '.' {
        map[pos.0][pos.1 - 1] = '@';
        map[pos.0][pos.1] = '.';
        pos.1 = pos.1 - 1;
    } else if left == '#' {
        // ignore
    } else if left == 'O' {
        // collect all items until . or #
        let mut items = vec![];
        items.push('O');
        let mut tmp_pos = pos.1 - 1; // above variable == tmp_pos
        loop {
            tmp_pos -= 1;
            let tmp_char = map[pos.0][tmp_pos];
            match tmp_char {
                '#' => {
                    items = vec![];
                    break;
                }
                '.' => break,
                'O' => items.push('O'),
                _ => panic!("alarm move up"),
            }
        }
        for (ind, item) in items.iter().enumerate() {
            map[pos.0][pos.1 - (ind + 2)] = *item;
        }
        if items.len() > 0 {
            map[pos.0][pos.1 - 1] = '@';
            map[pos.0][pos.1] = '.';
            pos.1 = pos.1 - 1;
        }
    }
    return (pos.0, pos.1);
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!();
}

fn get_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col == '@' {
                return (row_i, col_i);
            }
        }
    }
    println!("Could not find starting position");
    return (0, 0);
}
