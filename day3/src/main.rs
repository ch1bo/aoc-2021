use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn check_bit(number: &u64, position: usize) -> bool {
    number & (1 << position) != 0
}

fn main() -> io::Result<()> {
    // let file = File::open("input")?;
    // let reader = BufReader::new(file);
    // let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let input = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let entries: Vec<u64> = input.map(|l| u64::from_str_radix(l, 2).unwrap()).to_vec();
    println!("Input: {:?}", entries);

    // Part1
    let num_bits = input[0].len();
    let total = entries.len();
    let mut gamma: u32 = 0;
    let mut epsilon = 0;
    for i in 0..num_bits {
        let ones: usize = entries
            .iter()
            .map(|bits| if check_bit(bits, i) { 1 } else { 0 })
            .sum();
        if ones > total / 2 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }
    let part1 = gamma * epsilon;
    println!("Part1: {}", part1);

    // Part2
    let part2 = "";
    println!("Part2: {}", part2);

    Ok(())
}
