use std::io;
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::bytes::complete::tag;
use nom::Finish;

#[derive(Debug)]
enum Command {
    Forward (u64),
    Down (u64),
    Up (u64),
}

fn command(input: &str) -> nom::IResult<&str, Command> {
    nom::branch::alt((forward, down, up))(input)
}

// TODO(SN): pass enum constrctors to DRY this?
fn forward(input: &str) -> nom::IResult<&str, Command> {
    let (rest, value) = preceded(tag("forward "), digit1)(input)?;
    Ok((
      rest,
      Command::Forward(value.parse().unwrap()) // REVIEW(SN): mapping errors
    ))
}

fn down(input: &str) -> nom::IResult<&str, Command> {
    let (rest, value) = preceded(tag("down "), digit1)(input)?;
    Ok((
      rest,
      Command::Down(value.parse().unwrap()) // REVIEW(SN): mapping errors
    ))
}

fn up(input: &str) -> nom::IResult<&str, Command> {
    let (rest, value) = preceded(tag("up "), digit1)(input)?;
    Ok((
      rest,
      Command::Up(value.parse().unwrap()) // REVIEW(SN): mapping errors
    ))
}

fn main() -> io::Result<()> {
    // let file = File::open("input")?;
    // let reader = BufReader::new(file);
    // let input: Vec<i64> = reader
    //     .lines()
    //     .map(|l| l.unwrap().parse().unwrap())
    //     .collect();
    let input = ["forward 5" , "down 5" , "forward 8" , "up 3" , "down 8" , "forward 2"];
    let res = input.map(|i| command(i).finish().unwrap().1);
    println!("Input: {:?}", res);

    // Part1
    let part1 = "";
    println!("Part1: {}", part1);

    // Part2
    let part2 = "";
    println!("Part2: {}", part2);

    Ok(())
}
