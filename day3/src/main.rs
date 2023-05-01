use std::collections::HashSet;
use std::env;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input));

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input));
}

fn find_common_item(left_compartment: &str, right_compartment: &str) -> char {
    let mut left_compartment: Vec<_> = left_compartment.chars().collect();
    let mut right_compartment: Vec<_> = right_compartment.chars().collect();
    left_compartment.sort();
    left_compartment.dedup();
    right_compartment.sort();
    right_compartment.dedup();

    let mut left_ptr = 0;
    let mut right_ptr = 0;
    while left_ptr < left_compartment.len() && right_ptr < right_compartment.len() {
        let left_char = left_compartment[left_ptr];
        let right_char = right_compartment[right_ptr];

        if left_char == right_char {
            break;
        } else if left_char < right_char {
            left_ptr += 1;
        } else if left_char > right_char {
            right_ptr += 1;
        }
    }

    left_compartment[left_ptr]
}

fn get_badge<'a, I>(chunk: &mut I) -> char where I: Iterator<Item = &'a str> {
    let mut candidates: HashSet<_> = HashSet::from_iter(chunk.next().unwrap().chars());

    for line in chunk {
        let ruled_out: HashSet<_> = HashSet::from_iter(line.chars());
        candidates = candidates.intersection(&ruled_out).copied().collect();
    }

    candidates.into_iter().next().unwrap()
}

fn get_priority(item: char) -> usize {
    if 'a' <= item && item <= 'z' {
        return 1 + item as usize - 'a' as usize;
    } else if 'A' <= item && item <= 'Z' {
        return 27 + item as usize - 'A' as usize;
    } else {
        panic!("Invalid item code");
    }
}

fn a(input: String) -> usize {
    input.split("\n").map(|line| {
        let halfway_point = line.len() / 2;
        let (first_half, second_half) = (&line[..halfway_point], &line[halfway_point..]);
        get_priority(find_common_item(first_half, second_half))
    }).sum()
}

fn b(input: String) -> usize {
    input.split("\n").chunks(3).into_iter().map(|mut group| get_priority(get_badge(&mut group))).sum()
}

#[test]
fn test_get_priority_lowercase() {
    assert_eq!(get_priority('a'), 1);
    assert_eq!(get_priority('p'), 16);
    assert_eq!(get_priority('s'), 19);
    assert_eq!(get_priority('t'), 20);
    assert_eq!(get_priority('v'), 22);
    assert_eq!(get_priority('z'), 26);
}

#[test]
fn test_get_priority_uppercase() {
    assert_eq!(get_priority('A'), 27);
    assert_eq!(get_priority('L'), 38);
    assert_eq!(get_priority('P'), 42);
    assert_eq!(get_priority('Z'), 52);
}

#[test]
fn test_example_a() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(a(input), 157);
}

#[test]
fn test_example_b() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(b(input), 12);
}