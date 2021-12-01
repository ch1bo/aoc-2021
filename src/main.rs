use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let entries: Vec<i64> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    // Part1
    let mut last = 0;
    let mut increases = 0;
    for i in entries {
        if i > last && last != 0 {
            increases += 1;
        }
        last = i;
    }
    println!("{}", increases);

    Ok(())
}
