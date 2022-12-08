use std::{collections::HashSet, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
    println!("part2 {}", part2(&input));
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, window_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut marker = 0;
    for (i, window) in chars.windows(window_size).enumerate() {
        if HashSet::<&char>::from_iter(window).len() == window_size {
            marker = i + window_size;
            break;
        }
    }
    marker
}
