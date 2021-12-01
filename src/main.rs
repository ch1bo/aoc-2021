use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn increases(entries: &Vec<i64>) -> u64 {
    let mut last = 0;
    let mut res = 0;
    // REVIEW(SN): this is weird that I need to clone here
    for i in entries.clone() {
        if i > last && last != 0 {
            res += 1;
        }
        last = i;
    }
    res
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let entries: Vec<i64> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    // let entries = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    // Part1
    println!("Part1: {}", increases(&entries));

    // Part2
    let mut res: Vec<i64> = vec![];
    let mut window = vec![entries[1], entries[0]];
    for i in 2..entries.len() {
        window.insert(0, entries[i]);
        window.truncate(3);
        // REVIEW(SN): cloning here is likely unneeded?
        let s = window.clone().into_iter().sum();
        println!("{:?} -> {:?}", window, s);
        res.push(s);
    }
    println!("Part2: {}", increases(&res));

    Ok(())
}
