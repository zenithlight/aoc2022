use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input));

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input));
}

fn a(input: String) -> usize {
    // note to self: could use itertools::batching for the groups
    input.split("\n\n").map(|elf| {
        elf.split("\n").map(|snack| snack.parse::<usize>().unwrap()).sum()
    }).max().unwrap()
}

fn b(input: String) -> usize {
    let mut total_calories_by_elf: Vec<usize> = input.split("\n\n").map(|elf| {
        elf.split("\n").map(|snack| snack.parse::<usize>().unwrap()).sum()
    }).collect();
    total_calories_by_elf.sort();
    total_calories_by_elf.iter().rev().take(3).sum()
}

#[test]
fn test_example_a() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(a(input), 24000);
}

#[test]
fn test_example_b() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(b(input), 45000);
}