const INPUT_FILE: &str = "input/day09.txt";

type Memory = Vec<Block>;

#[derive(Debug, Clone, PartialEq)]
enum Block {
    Data(usize),
    Empty,
}

fn main() {
    println!("Day 9");

    // Read input and trim
    let input = std::fs::read_to_string(INPUT_FILE).expect("Could not read input");

    let memory = layout_memory(input.trim());

    // Part 1
    let compacted_1 = compact(&memory);
    let checksum_1 = checksum(&compacted_1);
    println!("[Part 1] {}", checksum_1);

    // Part 2
    let defragmented_2 = compact_defragment(&memory);
    let checksum_2 = checksum(&defragmented_2);
    println!("[Part 2] {}", checksum_2);
}

fn layout_memory(representation: &str) -> Memory {
    // The average size of the memory is 19/4 N
    let mut memory = Memory::with_capacity(19 * representation.len() / 4);

    for (i, ch) in representation.chars().enumerate() {
        let n = ch.to_digit(10).expect("Expected a digit");
        match i % 2 == 0 {
            true => (0..n).for_each(|_| memory.push(Block::Data(i >> 1))),
            false => (0..n).for_each(|_| memory.push(Block::Empty)),
        };
    }

    memory
}

fn compact(memory: &Memory) -> Memory {
    let mut compacted = memory.clone();

    let len = compacted.len();
    let empty: Vec<usize> = compacted
        .iter()
        .enumerate()
        .filter_map(|(i, block)| {
            if *block == Block::Empty {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let not_empty_rev: Vec<usize> = compacted
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(i, block)| {
            if *block != Block::Empty {
                Some((len - 1) - i)
            } else {
                None
            }
        })
        .collect();

    for (&e, &n) in empty.iter().zip(not_empty_rev.iter()) {
        if e >= n {
            break;
        }

        compacted.swap(e, n);
    }

    compacted
}

fn compact_defragment(memory: &Memory) -> Memory {
    todo!()
}

fn checksum(memory: &Memory) -> usize {
    memory
        .iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Data(id) => i * id,
            Block::Empty => 0,
        })
        .sum()
}

#[test]
fn test_example() {
    const INPUT: &str = "2333133121414131402";

    let memory = layout_memory(INPUT);

    let compacted_1 = compact(&memory);
    let checksum_1 = checksum(&compacted_1);
    assert_eq!(checksum_1, 1928);

    let defragmented_2 = compact_defragment(&memory);
    let checksum_2 = checksum(&defragmented_2);
    assert_eq!(checksum_2, 2858);
}
