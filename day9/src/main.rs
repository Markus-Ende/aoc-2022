use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
    println!("part2 {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut current_head = (0, 0);
    let mut current_tail = (0, 0);
    visited.insert(current_tail);

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        for _ in 0..steps.parse().unwrap() {
            let new_head = match direction {
                "U" => (current_head.0, current_head.1 - 1),
                "R" => (current_head.0 + 1, current_head.1),
                "D" => (current_head.0, current_head.1 + 1),
                "L" => (current_head.0 - 1, current_head.1),
                _ => panic!("unknown direction"),
            };
            if ((new_head.0 - current_tail.0) as i32).abs() >= 2
                || ((new_head.1 - current_tail.1) as i32).abs() >= 2
            {
                visited.insert(current_head);
                current_tail = current_head;
            }
            current_head = new_head;
        }
    }
    // println!("{:?}", visited);
    visited.len()
}

fn part2(_: &str) -> usize {
    0
}
