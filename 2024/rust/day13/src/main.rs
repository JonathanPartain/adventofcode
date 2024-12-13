use std::process::exit;

fn main() {
    let input = include_str!("../input.txt");
    let sections: Vec<&str> = input.split("\n\n").collect();
    part_one(&sections);
}

fn part_one(sections: &Vec<&str>) {
    let mut token_total = 0;
    for &section in sections.iter() {
        token_total += calc_section(section);
    }
    println!("Part 1: {}", token_total);
}
fn calc_section(section: &str) -> u32 {
    let mut a_x: u32 = 0;
    let mut a_y: u32 = 0;

    let mut b_x: u32 = 0;
    let mut b_y: u32 = 0;

    let mut score_x: u32 = 0;
    let mut score_y: u32 = 0;

    for (count, line) in section.lines().enumerate() {
        let coords: Vec<_> = line.split_whitespace().collect();
        match count {
            0 => {
                // A button
                a_x = coords[2]
                    .trim_end_matches(",")
                    .trim_start_matches("X+")
                    .parse::<u32>()
                    .unwrap();

                a_y = coords[3].trim_start_matches("Y+").parse::<u32>().unwrap();

                //println!("{:?}", coords);
            }
            1 => {
                // B button
                b_x = coords[2]
                    .trim_end_matches(",")
                    .trim_start_matches("X+")
                    .parse::<u32>()
                    .unwrap();

                b_y = coords[3].trim_start_matches("Y+").parse::<u32>().unwrap();
            }
            2 => {
                // prize info
                score_x = coords[1]
                    .trim_end_matches(",")
                    .trim_start_matches("X=")
                    .parse::<u32>()
                    .unwrap();

                score_y = coords[2].trim_start_matches("Y=").parse::<u32>().unwrap();
            }
            _ => {
                panic!("You split stuff wrong, exploding...")
            }
        }
    }

    let mut score_possibilities = vec![];
    for a_press in 1..=100 {
        for b_press in 1..=100 {
            if a_press * a_x + b_press * b_x == score_x && a_press * a_y + b_press * b_y == score_y
            {
                score_possibilities.push((a_press, b_press));
            }
        }
    }
    let mut cheapest = 0;
    if score_possibilities.len() > 0 {
        for s in score_possibilities {
            let tmp = s.0 * 3 + s.1;
            if cheapest == 0 || tmp < cheapest {
                cheapest = tmp
            }
        }
    }
    return cheapest;
}
