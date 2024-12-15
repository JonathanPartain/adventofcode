fn main() {
    println!("Hello, world!");
    let input = include_str!("../input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut map: Vec<Vec<char>> = sections[0].lines().map(|c| c.chars().collect()).collect();
    let instructions: Vec<char> = sections[1].chars().filter(|&c| c != '\n').collect();
    part_one(&mut map, &instructions);
}

fn part_one(map: &mut Vec<Vec<char>>, instr: &Vec<char>) {
    let (mut curr_row, mut curr_col) = get_start_position(map);
    print_map(map);
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
        //println!("row: {}, col: {}", curr_row, curr_col);
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
fn print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        println!("{:?}", row);
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
