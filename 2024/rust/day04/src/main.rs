use std::usize;

fn create_list(input: &str) -> Vec<Vec<&str>> {
    let mut outer: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        let inner: Vec<&str> = line.split("").filter(|c| *c != "").collect();
        outer.push(inner);
    }
    outer
}
fn main() {
    let input = include_str!("../input.txt");

    let vec_data = create_list(input);
    let vec_clone = vec_data.clone();
    find_words(vec_clone);

    find_x(vec_data);
}

fn check_word(chk_wrd: &str) -> bool {
    let find: &str = "XMAS";
    if chk_wrd == find || chk_wrd.chars().rev().collect::<String>() == find {
        return true;
    }
    false
}

fn find_words(matrix: Vec<Vec<&str>>) {
    let find = "XMAS";
    let word_len = find.len();
    let width = matrix[0].len();
    let height = matrix.len();
    let lookup = matrix.clone();

    let mut counter = 0;

    for (i_index, i_val) in matrix.into_iter().enumerate() {
        for (j_index, _) in i_val.into_iter().enumerate() {
            if j_index <= width - word_len && i_index <= height - word_len {
                // look diagonal down
                let mut diag_word = "".to_owned();
                diag_word.push_str(&lookup[i_index][j_index]);
                diag_word.push_str(&lookup[i_index + 1][j_index + 1]);
                diag_word.push_str(&lookup[i_index + 2][j_index + 2]);
                diag_word.push_str(&lookup[i_index + 3][j_index + 3]);
                if check_word(&diag_word) {
                    counter += 1;
                }
            }
            if j_index <= width - word_len && i_index >= word_len - 1 {
                // look diagonal up
                let mut diag_word = "".to_owned();
                diag_word.push_str(&lookup[i_index][j_index]);
                diag_word.push_str(&lookup[i_index - 1][j_index + 1]);
                diag_word.push_str(&lookup[i_index - 2][j_index + 2]);
                diag_word.push_str(&lookup[i_index - 3][j_index + 3]);
                if check_word(&diag_word) {
                    counter += 1;
                }
            }
            if j_index <= width - word_len {
                // look horizontal
                let mut horiz_word = "".to_owned();
                horiz_word.push_str(&lookup[i_index][j_index]);
                horiz_word.push_str(&lookup[i_index][j_index + 1]);
                horiz_word.push_str(&lookup[i_index][j_index + 2]);
                horiz_word.push_str(&lookup[i_index][j_index + 3]);
                if check_word(&horiz_word) {
                    counter += 1;
                }
            }
            if i_index <= height - word_len {
                // look vertical
                let mut vert_word = "".to_owned();
                vert_word.push_str(&lookup[i_index][j_index]);
                vert_word.push_str(&lookup[i_index + 1][j_index]);
                vert_word.push_str(&lookup[i_index + 2][j_index]);
                vert_word.push_str(&lookup[i_index + 3][j_index]);
                if check_word(&vert_word) {
                    counter += 1;
                }
            }
        }
    }
    println!("{}", counter);
}

fn is_xmas(i: &usize, j: &usize, mat: &Vec<Vec<&str>>) -> bool {
    // top left
    let tl = mat[i - 1][j - 1];
    // bot left
    let bl = mat[i + 1][j - 1];
    // top right
    let tr = mat[i - 1][j + 1];
    // bot righ
    let br = mat[i + 1][j + 1];

    if tl == "M" && br == "S" {
        if (bl == "M" && tr == "S") || (bl == "S" && tr == "M") {
            return true;
        }
    }
    if tl == "S" && br == "M" {
        if (bl == "M" && tr == "S") || (bl == "S" && tr == "M") {
            return true;
        }
    }
    false
}

fn find_x(matrix: Vec<Vec<&str>>) {
    let width = matrix[0].len();
    let height = matrix.len();
    let lookup = matrix.clone();

    let mut counter = 0;

    for (i_index, i_val) in matrix.into_iter().enumerate() {
        for (j_index, _) in i_val.into_iter().enumerate() {
            if i_index > 0 && i_index < width - 1 && j_index > 0 && j_index < height - 1 {
                // look for a
                let char: &str = &lookup[i_index][j_index];
                if char == "A" {
                    if is_xmas(&i_index, &j_index, &lookup) {
                        counter += 1
                    }
                }
            }
        }
    }
    println!("{}", counter);
}
