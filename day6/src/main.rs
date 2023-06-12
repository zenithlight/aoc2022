use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input).unwrap());

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input).unwrap());
}

fn flip(bitstring: u32, index: u32) -> u32 {
    bitstring ^ 1 << index
}

fn a(input: String) -> Result<usize, ()> {
    let mut bitstring: u32 = 0;
    let bytes = input.as_bytes();

    for i in 0..4 {
        bitstring = flip(bitstring, bytes[i] as u32 - 'a' as u32);
    }

    for i in 4..input.len() {
        let entering_alphabet_index = bytes[i] as u32 - 'a' as u32;
        let exiting_alphabet_index = bytes[i - 4] as u32 - 'a' as u32;

        bitstring = flip(bitstring, entering_alphabet_index);
        bitstring = flip(bitstring, exiting_alphabet_index);

        // if bit `i` in the bitstring is set, the `i`th letter has appeared an odd
        // number of times in the last 4 seen letters
        //
        // with a window of 4, if 4 different letters have been seen an odd number
        // of times, each of those letters must have been seen exactly once
        if bitstring.count_ones() == 4 {
            return Ok(i + 1);
        }
    }

    Err(())
}

fn b(input: String) -> Result<usize, ()> {
    let mut bitstring: u32 = 0;
    let bytes = input.as_bytes();

    for i in 0..14 {
        bitstring = flip(bitstring, bytes[i] as u32 - 'a' as u32);
    }

    for i in 14..input.len() {
        let entering_alphabet_index = bytes[i] as u32 - 'a' as u32;
        let exiting_alphabet_index = bytes[i - 14] as u32 - 'a' as u32;

        bitstring = flip(bitstring, entering_alphabet_index);
        bitstring = flip(bitstring, exiting_alphabet_index);

        // if bit `i` in the bitstring is set, the `i`th letter has appeared an odd
        // number of times in the last 4 seen letters
        //
        // with a window of 14, if 14 different letters have been seen an odd number
        // of times, each of those letters must have been seen exactly once
        if bitstring.count_ones() == 14 {
            return Ok(i + 1);
        }
    }

    Err(())
}

#[test]
fn test_example_a() {
    assert_eq!(a("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), Ok(5));
    assert_eq!(a("nppdvjthqldpwncqszvftbrmjlhg".to_string()), Ok(6));
    assert_eq!(a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), Ok(10));
    assert_eq!(a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), Ok(11));
}

#[test]
fn test_example_b() {
    assert_eq!(b("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), Ok(19));
    assert_eq!(b("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), Ok(23));
    assert_eq!(b("nppdvjthqldpwncqszvftbrmjlhg".to_string()), Ok(23));
    assert_eq!(b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), Ok(29));
    assert_eq!(b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), Ok(26));
}