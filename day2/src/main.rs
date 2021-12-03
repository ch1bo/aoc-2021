use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::Finish;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

fn command(input: &str) -> nom::IResult<&str, Command> {
    nom::branch::alt((forward, down, up))(input)
}

// TODO(SN): pass enum constrctors to DRY this?
fn forward(input: &str) -> nom::IResult<&str, Command> {
    let (rest, value) = preceded(tag("forward "), digit1)(input)?;
    Ok((
        rest,
        Command::Forward(value.parse().unwrap()), // REVIEW(SN): mapping errors
    ))
}

fn down(input: &str) -> nom::IResult<&str, Command> {
    let (rest, value) = preceded(tag("down "), digit1)(input)?;
    Ok((
        rest,
        Command::Down(value.parse().unwrap()), // REVIEW(SN): mapping errors
    ))
}

fn up(input: &str) -> nom::IResult<&str, Command> {
    let (rest, value) = preceded(tag("up "), digit1)(input)?;
    Ok((
        rest,
        Command::Up(value.parse().unwrap()), // REVIEW(SN): mapping errors
    ))
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // let input = [
    //     "forward 5",
    //     "down 5",
    //     "forward 8",
    //     "up 3",
    //     "down 8",
    //     "forward 2",
    // ];
    let commands = input
        .iter()
        .map(|i| command(i).finish().unwrap().1)
        .collect::<Vec<_>>();
    println!("Input: {:?}", commands);

    // Part1
    let mut pos = (0, 0);
    for c in commands.iter() {
        match c {
            Command::Forward(i) => {
                pos = (pos.0 + i, pos.1);
            }
            Command::Down(i) => {
                pos = (pos.0, pos.1 + i);
            }
            Command::Up(i) => {
                pos = (pos.0, pos.1 - i);
            }
        }
    }
    let part1 = pos.0 * pos.1;
    println!("Part1: {}", part1);

    // Part2
    let (a, b, _) = commands.iter().fold((0, 0, 0), |acc, item| match item {
        Command::Forward(i) => (acc.0 + i, acc.1 + acc.2 * i, acc.2),
        Command::Down(i) => (acc.0, acc.1, acc.2 + i),
        Command::Up(i) => (acc.0, acc.1, acc.2 - i),
    });
    let part2 = a * b;
    println!("Part2: {}", part2);

    Ok(())
}
