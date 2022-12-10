use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1    {}", part1(&input));
    println!("part2:");
    part2(&input);
}

fn part1(input: &str) -> i32 {
    let mut signal_strength = 0;
    let mut calculate_signal_strength = |cycle, x| {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            // println!("cycle {}, x {}", cycle, x);
            signal_strength += cycle * x;
        }
    };

    run_program(input, &mut calculate_signal_strength);
    signal_strength
}

fn part2(input: &str) -> () {
    fn draw(cycle: i32, x: i32) -> () {
        if [x - 1, x, x + 1].contains(&((cycle - 1) % 40)) {
            print!("#");
        } else {
            print!(".");
        }
        check_newline(cycle);
    }

    fn check_newline(cycle: i32) -> () {
        if [40, 80, 120, 160, 200, 240].contains(&cycle) {
            println!();
        }
    }

    run_program(input, &mut draw);
}

fn run_program(input: &str, tick: &mut dyn FnMut(i32, i32) -> ()) -> () {
    let mut x = 1;
    let mut cycle = 1;

    input.lines().for_each(|line| {
        let instruction: Vec<_> = line.split_ascii_whitespace().collect();
        match &instruction[..] {
            ["noop"] => {
                tick(cycle, x);
                cycle += 1;
            }
            ["addx", arg] => {
                tick(cycle, x);
                cycle += 1;
                tick(cycle, x);
                cycle += 1;
                x += arg.parse::<i32>().unwrap();
            }
            _ => panic!("unknown instruction"),
        }
    });
}
