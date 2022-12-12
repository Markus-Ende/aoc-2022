extern crate pathfinding;

use pathfinding::matrix::Matrix;
use pathfinding::prelude::dijkstra;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "test-input.txt"
    };

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1    {}", part1(&input));
    println!("part2    {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    find_path(input)
}

fn part2(input: &str) -> i32 {
    let cleaned_input = input.replace("S", "a");
    let mut input_vec: Vec<String> = vec![];
    for i in 0..cleaned_input.len() {
        if cleaned_input.chars().nth(i).unwrap() == 'a' {
            let mut s = cleaned_input.clone();
            s.replace_range(i..=i, "S");
            input_vec.push(s);
        }
    }

    // TODO: increase performance by providing current min path and terminating graph algorithm early if it exceeds it
    let mut path_lengths: Vec<_> = input_vec
        .iter()
        .map(|input| find_path(input))
        .filter(|result| *result >= 0)
        .collect();
    path_lengths.sort();
    path_lengths[0]
}

fn find_path(input: &str) -> i32 {
    let height_map_raw = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    let start = height_map_raw
        .indices()
        .find(|i| *height_map_raw.get(*i).unwrap() == 'S')
        .unwrap();
    let goal = height_map_raw
        .indices()
        .find(|i| *height_map_raw.get(*i).unwrap() == 'E')
        .unwrap();

    let height_map = height_map_raw.map(|c| {
        if c == 'S' {
            'a'
        } else if c == 'E' {
            'z'
        } else {
            c
        }
    });

    let result = dijkstra(
        &start,
        |&(row, col)| {
            height_map
                .neighbours((row, col), false)
                .filter(|neighbour| {
                    (*height_map.get(*neighbour).unwrap() as i32)
                        - (*height_map.get((row, col)).unwrap() as i32)
                        <= 1
                })
                .map(|n| (n, 1))
                .collect::<Vec<_>>()
        },
        |p| *p == goal,
    );

    if let Some(path) = result {
        path.1
    } else {
        -1
    }
}
