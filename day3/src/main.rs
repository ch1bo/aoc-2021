use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // let input = [
    //     "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    //     "00010", "01010",
    // ];
    println!("Input: {:?}", input);

    // Part1
    let total = input.len();
    let num_bits = input[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..num_bits {
        let count: usize = input
            .iter()
            .map(|bits| {
                if bits.chars().nth(i) == Some('1') {
                    1
                } else {
                    0
                }
            })
            .sum();
        if count > total / 2 {
            gamma = (gamma + 1) << 1;
            epsilon = epsilon << 1;
        } else {
            gamma = gamma << 1;
            epsilon = (epsilon + 1) << 1;
        }
    }
    let part1 = (gamma >> 1) * (epsilon >> 1);
    println!("Part1: {}", part1);

    // Part2
    let part2 = "";
    println!("Part2: {}", part2);

    Ok(())
}
