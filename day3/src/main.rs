use crate::Mode::MostCommon;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn check_bit(number: &u64, position: usize) -> bool {
    number & (1 << position) != 0
}

fn count_bit(numbers: &[u64], position: usize) -> usize {
    numbers
        .iter()
        .map(|bits| if check_bit(bits, position) { 1 } else { 0 })
        .sum()
}

#[derive(Debug)]
enum Mode {
    MostCommon,
    LeastCommon,
}

fn go_part2(entries: Vec<u64>, index: usize, mode: &Mode) -> Option<u64> {
    let total = entries.len();
    let ones = count_bit(&entries, index);
    let search = match mode {
        Mode::MostCommon => {
            if ones > total - ones {
                true
            } else if ones == total - ones {
                true
            } else {
                false
            }
        }
        Mode::LeastCommon => {
            // println!("ones {} < {} / 2 ({})", ones, total, total / 2);
            if ones < total - ones {
                true
            } else if ones == total - ones {
                false
            } else {
                false
            }
        }
    };
    // println!("index {} {:?} -> {}", index, mode, search);
    let remaining = entries
        .into_iter()
        .filter(|x| check_bit(&x, index) == search)
        .collect::<Vec<u64>>();
    // println!("remaining");
    // for i in &remaining {
    //     println!("  {:05b}", i);
    // }
    if remaining.len() <= 1 {
        return remaining.last().cloned();
    } else {
        // NOTE(SN): could underflow
        return go_part2(remaining, index - 1, mode);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // let input = [
    //     "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    //     "00010", "01010",
    // ];
    let entries: Vec<u64> = input
        .iter()
        .map(|l| u64::from_str_radix(l, 2).unwrap())
        .collect();
    println!("Input:");
    for i in &entries {
        println!("  {:05b}", i);
    }

    // Part1
    let num_bits = input[0].len();
    let total = entries.len();
    let mut gamma: u32 = 0;
    let mut epsilon = 0;
    for i in 0..num_bits {
        let ones = count_bit(&entries, i);
        if ones > total / 2 {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }
    let part1 = gamma * epsilon;
    println!("Part1: {}", part1);

    // Part2
    let oxygen = go_part2(entries.clone(), num_bits - 1, &Mode::MostCommon);
    let co2 = go_part2(entries.clone(), num_bits - 1, &Mode::LeastCommon);
    println!("Part2: {:?}", oxygen.unwrap() * co2.unwrap());

    Ok(())
}
