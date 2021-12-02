use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    // let input: Vec<i64> = reader
    //     .lines()
    //     .map(|l| l.unwrap().parse().unwrap())
    //     .collect();
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    println!("Input: {:?}", input);

    // Part1
    let part1 = "";
    println!("Part1: {}", part1);

    // Part2
    let part2 = "";
    println!("Part2: {}", part2);

    Ok(())
}
