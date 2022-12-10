use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1    {}", part1(&input));
    println!("part2    {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycles = 0;
    let mut signal_strength = 0;

    input.lines().for_each(|line| {
        let instruction: Vec<_> = line.split_ascii_whitespace().collect();
        match &instruction[..] {
            ["noop"] => {
                cycles += 1;
                if [20, 60, 100, 140, 180, 220].contains(&cycles) {
                    println!("cycle {}, x {}", cycles, x);
                    signal_strength += cycles * x;
                }
            }
            ["addx", arg] => {
                cycles += 2;
                if [20, 21, 60, 61, 100, 101, 140, 141, 180, 181, 220, 221].contains(&cycles) {
                    let real_cycle = if cycles % 2 == 0 { cycles } else { cycles - 1 };
                    println!("cycle {}, x {}", real_cycle, x);
                    signal_strength += real_cycle * x;
                }
                x += arg.parse::<i32>().unwrap();
            }
            _ => panic!("unknown instruction"),
        }
    });
    signal_strength
}

fn part2(_: &str) -> usize {
    0
}
