const INPUT_FILE: &str = "input/day04.txt";

type Block = Vec<Vec<char>>;

fn main() {
    println!("Day 4");

    let input_block = read_block(INPUT_FILE).expect("Failed to read input file");

    // Part 1: XMAS
    let num_xmas = count_xmas_in_all_directions(&input_block);
    println!("[Part 1] {}", num_xmas);

    /* Part 2: Count X-MAS pattern in X match_x_mas_pattern
     * Siempre me hacen el lío con la segunda parte. ¡Qué hijos de puta!
     */
    let num_x_mas = count_x_mas_patterns(&input_block);
    println!("[Part 2] {}", num_x_mas);
}

fn count_xmas_in_all_directions(block: &Block) -> u32 {
    let mut count = 0;

    let width = block[0].len() as i32;
    let height = block.len() as i32;

    for row in 0..height {
        for col in 0..width {
            count += count_word_matches_in_all_directions("XMAS", block, (row, col));
        }
    }

    count
}

fn count_word_matches_in_all_directions(word: &str, block: &Block, pos: (i32, i32)) -> u32 {
    const ALL_DIRS: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut count = 0;

    for dir in ALL_DIRS {
        if match_word_in_direction(word, block, pos, dir) {
            count += 1;
        }
    }

    count
}

fn match_word_in_direction(word: &str, block: &Block, pos: (i32, i32), dir: (i32, i32)) -> bool {
    if let "" = word {
        return true;
    }

    let height = block[0].len() as i32;
    let width = block.len() as i32;

    let out_of_bounds = (pos.0 < 0) || (pos.0 >= height) || (pos.1 < 0) || (pos.1 >= width);
    if let true = out_of_bounds {
        return false;
    }

    let first_letter: char = word.chars().nth(0).unwrap();
    if first_letter == block[pos.1 as usize][pos.0 as usize] {
        return match_word_in_direction(&word[1..], block, (pos.0 + dir.0, pos.1 + dir.1), dir);
    }

    false
}

fn count_x_mas_patterns(block: &Block) -> u32 {
    let mut count = 0;

    let width = block[0].len() as i32;
    let height = block.len() as i32;

    for row in 0..height {
        for col in 0..width {
            if match_x_mas_pattern(block, (row, col)) {
                count += 1;
            }
        }
    }

    count
}

fn match_x_mas_pattern(block: &Block, pos: (i32, i32)) -> bool {
    const TARGET: &str = "MAS";
    assert_eq!(TARGET.len() & 1, 1);

    let height = block.len() as i32;
    let width = block[0].len() as i32;

    let out_of_bounds =
        (pos.0 <= 0) || ((pos.0 + 1) >= height) || (pos.1 <= 0) || ((pos.1 + 1) >= width);
    if out_of_bounds {
        return false;
    }

    // Don't waste time if the central letter is not an A
    if block[pos.0 as usize][pos.1 as usize] != 'A' {
        return false;
    }

    // Strings to check (X)
    let b00: String = [
        block[(pos.0 - 1) as usize][(pos.1 + 1) as usize],
        block[(pos.0) as usize][(pos.1) as usize],
        block[(pos.0 + 1) as usize][(pos.1 - 1) as usize],
    ]
    .iter()
    .collect();
    let b01: String = b00.chars().rev().collect();
    let b10: String = [
        block[(pos.0 - 1) as usize][(pos.1 - 1) as usize],
        block[(pos.0) as usize][(pos.1) as usize],
        block[(pos.0 + 1) as usize][(pos.1 + 1) as usize],
    ]
    .iter()
    .collect();
    let b11: String = b10.chars().rev().collect();

    // Match pattern
    ((b00 == TARGET) || (b01 == TARGET)) && ((b10 == TARGET) || (b11 == TARGET))
}

fn read_input(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(&file_path)?)
}

fn read_block(file_path: &str) -> Result<Block, Box<dyn std::error::Error>> {
    let text = read_input(file_path)?;
    str_to_block(&text)
}

fn str_to_block(text: &str) -> Result<Block, Box<dyn std::error::Error>> {
    let mut block = Block::with_capacity(140);

    for line in text.lines() {
        let mut char_line: Vec<char> = Vec::with_capacity(140);
        for ch in line.chars() {
            char_line.push(ch);
        }

        block.push(char_line);
    }

    Ok(block)
}

#[test]
fn test_examples() {
    const TEXT_0: &str = "MMMSXXMASM\n\
                          MSAMXMSMSA\n\
                          AMXSXMAAMM\n\
                          MSAMASMSMX\n\
                          XMASAMXAMM\n\
                          XXAMMXXAMA\n\
                          SMSMSASXSS\n\
                          SAXAMASAAA\n\
                          MAMMMXMMMM\n\
                          MXMXAXMASX";

    let block_0 = str_to_block(TEXT_0).unwrap();
    assert_eq!(count_xmas_in_all_directions(&block_0), 18);
}
