use std::{env, fmt, fs};

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64, &Vec<Monkey>) -> i64>,
    test: Box<dyn Fn(i64) -> usize>,
    inspect_count: usize,
    module: i64,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspect_count", &self.inspect_count)
            .field("module", &self.module)
            .finish()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "input.txt"
    };

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("part1 {}", monkey_business(&input, 20, true));
    println!("part2 {}", monkey_business(&input, 10000, false));
}

fn monkey_business(input: &str, rounds: i32, divide_by_3_after_operation: bool) -> usize {
    let mut monkeys = parse_monkeys(input, divide_by_3_after_operation);

    for _round in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let items = &monkeys[monkey_index].items.to_owned();
            (&mut monkeys[monkey_index]).inspect_count += items.len();
            let _ = &monkeys[monkey_index].items.clear();
            for item in items {
                let monkey = &monkeys[monkey_index];
                let worry = (monkey.operation)(*item, &monkeys);

                let target_monkey_index = (monkey.test)(worry);

                let target_monkey = &mut monkeys[target_monkey_index];
                target_monkey.items.push(worry);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    let inspects: Vec<usize> = monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspect_count)
        .collect();
    inspects[0] * inspects[1]
}

fn parse_monkeys(input: &str, divide_by_3_after_operation: bool) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|x| {
            let mut starting_items: Option<Vec<i64>> = None;
            let mut operation_op: Option<String> = None;
            let mut operation_operand: Option<String> = None;
            let mut module = 0;
            let mut true_monkey = 0;
            let mut false_monkey = 0;

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
                        module = divisor.parse().unwrap();
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

            Monkey {
                items: starting_items.unwrap(),
                operation: Box::new(move |old, all_monkeys| {
                    let op = &operation_op.clone().unwrap()[..];
                    let operand = &operation_operand.clone().unwrap()[..];
                    let mut new_value = match (op, operand) {
                        ("*", "old") => old * old,
                        ("*", operand) => old * operand.parse::<i64>().unwrap(),
                        ("+", operand) => old + operand.parse::<i64>().unwrap(),
                        _ => panic!(),
                    };
                    if divide_by_3_after_operation {
                        new_value = new_value / 3
                    }

                    let mut residues: Vec<i64> = vec![];
                    let mut modulii: Vec<i64> = vec![];
                    for i in 0..all_monkeys.len() {
                        residues.push(new_value % all_monkeys[i].module);
                        modulii.push(all_monkeys[i].module);
                    }
                    chinese_remainder(&residues, &modulii).unwrap()
                }),
                test: Box::new(move |worry_level| {
                    if worry_level % module == 0 {
                        true_monkey
                    } else {
                        false_monkey
                    }
                }),
                inspect_count: 0,
                module,
            }
        })
        .collect()
}

// chinese remainder theorem implementation from
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
// because I was too lazy to write it by myself...

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
