use std::{collections::HashSet, env, fs, iter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // println!("Input file {}", file_path);

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1    {}", part1(&input));
    println!("part1 v2 {}", part1_v2(&input));
    println!("part2    {}", part2(&input));
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

fn part1_v2(input: &str) -> usize {
    simulate_rope(input, 2)
}

fn part2(input: &str) -> usize {
    simulate_rope(input, 10)
}

fn simulate_rope(input: &str, size: usize) -> usize {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut segments = iter::repeat((0, 0)).take(size).collect::<Vec<(_, _)>>();
    visited.insert((0, 0));

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        for _ in 0..steps.parse().unwrap() {
            let head = segments.get_mut(0).unwrap();
            match direction {
                "U" => head.1 -= 1,
                "R" => head.0 += 1,
                "D" => head.1 += 1,
                "L" => head.0 -= 1,
                _ => panic!("unknown direction"),
            };
            for i in 1..segments.len() {
                let head_to_compare = segments[i - 1];
                let tail_to_compare = segments.get_mut(i).unwrap();
                let dx = head_to_compare.0 - tail_to_compare.0 as i32;
                let dy = head_to_compare.1 - tail_to_compare.1 as i32;

                if dx.abs() > 1 || dy.abs() > 1 {
                    tail_to_compare.0 += dx.signum();
                    tail_to_compare.1 += dy.signum();
                }
            }
            visited.insert(*segments.last().unwrap());

            // println!("{:?}", segments);
        }
    }
    // println!("{:?}", visited);
    visited.len()
}
