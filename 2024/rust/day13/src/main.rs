use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("../input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect();
    part_one(&sections);
    part_two(&sections);
    println!("Time elapsed: {:.3?}", start.elapsed());
}

fn part_one(sections: &Vec<&str>) {
    let mut token_total = 0;
    for &section in sections.iter() {
        token_total += calc_section(section, false);
    }
    println!("Part 1: {}", token_total);
}
fn part_two(sections: &Vec<&str>) {
    let mut token_total = 0;
    for &section in sections.iter() {
        token_total += calc_section(section, true);
    }
    println!("Part 2: {}", token_total);
}
fn calc_section(section: &str, is_part_2: bool) -> i128 {
    let mut a_x: i128 = 0;
    let mut a_y: i128 = 0;

    let mut b_x: i128 = 0;
    let mut b_y: i128 = 0;

    let mut score_x: i128 = 0;
    let mut score_y: i128 = 0;

    for (count, line) in section.lines().enumerate() {
        let coords: Vec<_> = line.split_whitespace().collect();
        match count {
            0 => {
                // A button
                a_x = coords[2]
                    .trim_end_matches(",")
                    .trim_start_matches("X+")
                    .parse::<i128>()
                    .unwrap();

                a_y = coords[3].trim_start_matches("Y+").parse::<i128>().unwrap();

                //println!("{:?}", coords);
            }
            1 => {
                // B button
                b_x = coords[2]
                    .trim_end_matches(",")
                    .trim_start_matches("X+")
                    .parse::<i128>()
                    .unwrap();

                b_y = coords[3].trim_start_matches("Y+").parse::<i128>().unwrap();
            }
            2 => {
                // prize info
                score_x = coords[1]
                    .trim_end_matches(",")
                    .trim_start_matches("X=")
                    .parse::<i128>()
                    .unwrap();

                score_y = coords[2].trim_start_matches("Y=").parse::<i128>().unwrap();
            }
            _ => {
                panic!("You split stuff wrong, exploding...")
            }
        }
    }
    if is_part_2 {
        score_x += 10_000_000_000_000;
        score_y += 10_000_000_000_000;
    }
    let det = (b_x * a_y) - (a_x * b_y);
    if det == 0 {
        return 0;
    }

    // math, thanks chatgpt, ddg and kristoffer
    let a_press = ((b_x * score_y) - (b_y * score_x)) / det;
    let b_press = (score_x - (a_press * a_x)) / b_x;

    if a_press * a_x + b_press * b_x == score_x && a_press * a_y + b_press * b_y == score_y {
        return a_press * 3 + b_press;
    }
    return 0;
}
