extern crate pathfinding;

use pathfinding::grid;
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
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

fn part1(input: &str) -> usize {
    let rows: Vec<&str> = input.lines().collect();

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

    println!("map: {:?}", height_map);
    println!("start: {:?}, goal: {:?}", start, goal);

    let result = dijkstra(
        &start,
        |&(row, col)| {
            let current = height_map.get((row, col)).unwrap();
            println!(
                "checking {:?}: {} with neighbours {:?}",
                (row, col),
                current,
                height_map.neighbours((row, col), false).collect::<Vec<_>>()
            );

            let reachable_neighbours = height_map
                .neighbours((row, col), false)
                .filter(|neighbour| {
                    print!(
                        "checking neighbour {:?}: {}/{}.. ",
                        neighbour,
                        height_map.get(*neighbour).unwrap(),
                        *height_map.get(*neighbour).unwrap() as i32
                    );
                    let is_reachable =
                        (*height_map.get(*neighbour).unwrap() as i32) - (*current as i32) <= 1;
                    println!(
                        "{:?}: {}, is_reachable: {}",
                        neighbour,
                        height_map.get(*neighbour).unwrap(),
                        is_reachable
                    );
                    is_reachable
                })
                .map(|n| (n, 1))
                .collect::<Vec<_>>();

            println!(
                "reachable neighbours of {:?}: {:?}",
                (row, col),
                reachable_neighbours
            );
            reachable_neighbours
        },
        |p| *p == goal,
    )
    .unwrap();

    println!("result {:?}", result);
    result.1
}
