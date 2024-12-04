fn calc_score(elf_input: &str, your_input: &str) -> u32 {
    // check wins and draws, else 0
    let points = match elf_input {
        "A" =>
        // elf rock
        {
            match your_input {
                "X" => 3, // lose + scissors 0 + 3
                "Y" => 4, // draw + rock 3 + 1
                "Z" => 8, // win + paper 6 + 2
                _ => panic!("How?"),
            }
        }
        "B" =>
        // elf paper
        {
            match your_input {
                "X" => 1, // loss + rock 0 + 1
                "Y" => 5, // draw + paper 3 + 2
                "Z" => 9, // win + scissors 6 + 3
                _ => panic!("How?"),
            }
        }
        "C" =>
        // elf scissors
        {
            match your_input {
                "X" => 2, // lose + paper 0 + 2
                "Y" => 6, // draw + scissors 3 + 3
                "Z" => 7, // win + rock 6 + 1
                _ => panic!("How?"),
            }
        }
        _ => panic!("How"),
    };

    points
}

fn main() {
    // opponent_choice my_hoice
    // A -> Rock
    // B -> Paper
    // C -> Scissors
    //
    // X -> Rock : 1 point
    // Y -> Paper : 2 points
    // Z -> Scissors : 3 points
    //
    // Win -> 6 points
    // Draw -> 3 points
    // Lostt -> 0 points
    let input = include_str!("../input.txt");
    let mut points_total = 0;
    for line in input.lines() {
        if let Some((elf, me)) = line.split_once(" ") {
            points_total += calc_score(elf, me);
        }
    }
    println!("{points_total}");
}
