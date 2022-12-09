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
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut coordinates_of_visible_trees = HashSet::<(usize, usize)>::new();

    for x in 1..map[0].len() - 1 {
        let mut current_height = map[0][x];

        for y in 1..map.len() - 1 {
            if map[y][x] > current_height {
                coordinates_of_visible_trees.insert((x, y));
                current_height = map[y][x];
            }
        }

        let mut current_height = map.last().unwrap()[x];
        for y in (1..map.len() - 1).rev() {
            if map[y][x] > current_height {
                coordinates_of_visible_trees.insert((x, y));
                current_height = map[y][x];
            }
        }
    }

    for y in 1..map.len() - 1 {
        let mut current_height = map[y][0];
        for x in 1..map[0].len() - 1 {
            if map[y][x] > current_height {
                coordinates_of_visible_trees.insert((x, y));
                current_height = map[y][x];
            }
        }

        let mut current_height = *map[y].last().unwrap();
        for x in (1..map[0].len() - 1).rev() {
            if map[y][x] > current_height {
                coordinates_of_visible_trees.insert((x, y));
                current_height = map[y][x];
            }
        }
    }

    // println!("{:?}", coordinates_of_visible_trees);
    let trees_on_edge = 2 * map.len() + 2 * map[0].len() - 4;
    // println!("trees_on_edge {}", trees_on_edge);
    trees_on_edge + coordinates_of_visible_trees.len()
}

fn part2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut max_score = 0;

    for x in 1..map[0].len() - 1 {
        for y in 1..map.len() - 1 {
            let mut tree_count_up = 0;
            for dy in (0..y).rev() {
                tree_count_up += 1;
                if map[dy][x] >= map[y][x] {
                    break;
                }
            }

            let mut tree_count_down = 0;
            for dy in (y + 1)..map.len() {
                tree_count_down += 1;
                if map[dy][x] >= map[y][x] {
                    break;
                }
            }

            let mut tree_count_left = 0;
            for dx in (0..x).rev() {
                tree_count_left += 1;
                if map[y][dx] >= map[y][x] {
                    break;
                }
            }

            let mut tree_count_right = 0;
            for dx in ((x + 1)..map.len()) {
                tree_count_right += 1;
                if map[y][dx] >= map[y][x] {
                    break;
                }
            }

            let score = tree_count_up * tree_count_down * tree_count_left * tree_count_right;
            if score > max_score {
                max_score = score;
            }

            println!(
                "({},{}): {} (up {}, right {}, down {}, left {})",
                x, y, score, tree_count_up, tree_count_right, tree_count_down, tree_count_left
            );
        }
    }

    max_score
}
