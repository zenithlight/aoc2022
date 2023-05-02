use std::env;
use std::convert::From;

struct Shift(usize, usize);

impl Shift {
    fn contains(&self, other: &Shift) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    fn overlaps(&self, other: &Shift) -> bool {
        self.1 >= other.0 && other.1 >= self.0
    }
}

impl From<&str> for Shift {
    fn from(code: &str) -> Self {
        let mut split_code = code.split("-");
        Self(
            split_code.next().unwrap().parse().unwrap(),
            split_code.next().unwrap().parse().unwrap()
        )
    }
}

struct ShiftPair(Shift, Shift);

impl From<&str> for ShiftPair {
    fn from(code: &str) -> Self {
        let mut split_code = code.split(",");
        Self(
            Shift::from(split_code.next().unwrap()),
            Shift::from(split_code.next().unwrap())
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input));

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input));
}

fn a(input: String) -> usize {
    input.split("\n").map(ShiftPair::from).filter(
        |shift_pair| shift_pair.0.contains(&shift_pair.1) || shift_pair.1.contains(&shift_pair.0)
    ).count()
}

fn b(input: String) -> usize {
    input.split("\n").map(ShiftPair::from).filter(
        |shift_pair| shift_pair.0.overlaps(&shift_pair.1)
    ).count()
}

#[test]
fn test_example_a() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(a(input), 2);
}

#[test]
fn test_example_b() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(b(input), 4);
}