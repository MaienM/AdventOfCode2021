use std::env;
use std::fs;
use std::panic;

type Runnable = fn(String) -> i32;

pub fn run(part1: Runnable, part2: Runnable) {
    let args: Vec<String> = env::args().collect();

    let name = args[0]
        .split("/")
        .last()
        .expect("Unable to determine binary name.");

    let filename = format!("inputs/{}.txt", name);
    let filename = args.get(1).unwrap_or(&filename);

    println!("Running {} with input {}...", name, filename);

    let input = fs::read_to_string(filename).expect("Unable to read input file.");

    let result1 = part1(input.to_string());
    println!("> Part 1: {}", result1);

    if part2 == missing {
        println!("> Part 2 is not implemented.");
    } else {
        let result2 = part2(input);
        println!("> Part 2: {}", result2);
    }
}

pub fn missing(_data: String) -> i32 {
    return -1;
}

pub fn parse_list_of_numbers(input: String) -> Vec<i32> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Unable to parse line {:?}.", line))
        })
        .collect();
}
