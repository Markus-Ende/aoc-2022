use std::{env, fmt, fs, io};

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> usize>,
    inspect_count: usize,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspect_count", &self.inspect_count)
            .finish()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "test-input.txt"
    };

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);

    for round in 0..20 {
        println!("Round {}", round);
        for monkey_index in 0..monkeys.len() {
            let items = &monkeys[monkey_index].items.to_owned();
            println!("Monkey {} inspecting items {:?}", monkey_index, items);
            (&mut monkeys[monkey_index]).inspect_count += items.len();
            let _ = &monkeys[monkey_index].items.clear();
            for item in items {
                let monkey = &monkeys[monkey_index];
                let worry = (monkey.operation)(*item) / 3;
                println!("  item {} -> {}", item, worry);
                let target_monkey_index = (monkey.test)(worry);
                let target_monkey = &mut monkeys[target_monkey_index];
                println!(
                    "  target monkey {}: {:?}",
                    target_monkey_index, target_monkey
                );
                target_monkey.items.push(worry);
                println!("  target monkey {:?}", target_monkey_index);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    println!("{:?}", monkeys);
    let inspects: Vec<usize> = monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspect_count)
        .collect();
    inspects[0] * inspects[1]
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|x| {
            let mut starting_items: Option<Vec<i32>> = None;
            let mut operation_op: Option<String> = None;
            let mut operation_operand: Option<String> = None;
            let mut divisible_by = -1;
            let mut true_monkey = 0;
            let mut false_monkey = 0;

            // println!("{}", x);
            x.lines().for_each(|line| {
                let spec: Vec<_> = line.trim().split_ascii_whitespace().collect();
                match &spec[..] {
                    ["Monkey", ..] => {} //noop,
                    ["Starting", "items:", items @ ..] => {
                        starting_items = Some(
                            items
                                .iter()
                                .map(|item| item.replace(",", "").parse().unwrap())
                                .collect(),
                        );
                    }
                    ["Operation:", "new", "=", "old", op, operand] => {
                        operation_op = Some(String::from(*op));
                        operation_operand = Some(String::from(*operand));
                    }
                    ["Test:", .., divisor] => {
                        divisible_by = divisor.parse().unwrap();
                    }
                    ["If", "true:", .., target_monkey] => {
                        true_monkey = target_monkey.parse().unwrap();
                    }
                    ["If", "false:", .., target_monkey] => {
                        false_monkey = target_monkey.parse().unwrap();
                    }

                    x => panic!("unknown instruction {:?}", x),
                }
            });

            /*
            let mut starting_items: Option<Vec<i32>> = None;
            let mut operation_op: Option<String> = None;
            let mut operation_operand: Option<String> = None;
            let mut divisible_by = -1;
            let mut true_monkey = 0;
            let mut false_monkey = 0; */
            println!(
                "Parsed: {}, {}, {}",
                divisible_by, true_monkey, false_monkey
            );

            Monkey {
                items: starting_items.unwrap(),
                operation: Box::new(move |old| {
                    let op = &operation_op.clone().unwrap()[..];
                    let operand = &operation_operand.clone().unwrap()[..];
                    match (op, operand) {
                        ("*", "old") => old * old,
                        ("*", operand) => old * operand.parse::<i32>().unwrap(),
                        ("+", operand) => old + operand.parse::<i32>().unwrap(),
                        _ => panic!(),
                    }
                }),
                test: Box::new(move |worry_level| {
                    if worry_level % divisible_by == 0 {
                        true_monkey
                    } else {
                        false_monkey
                    }
                }),
                inspect_count: 0,
            }
        })
        .collect()
}
