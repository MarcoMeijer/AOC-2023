use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::satisfy, is_digit},
    combinator::success,
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

fn digit(input: &str) -> IResult<&str, char> {
    alt((
        satisfy(|c| is_digit(c as u8)),
        preceded(tag("one"), success('1')),
        preceded(tag("two"), success('2')),
        preceded(tag("three"), success('3')),
        preceded(tag("four"), success('4')),
        preceded(tag("five"), success('5')),
        preceded(tag("six"), success('6')),
        preceded(tag("seven"), success('7')),
        preceded(tag("eight"), success('8')),
        preceded(tag("nine"), success('9')),
    ))(input)
}

fn part2(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for i in 0..line.len() {
            if let Ok((_, c)) = digit(&line[i..]) {
                if first.is_none() {
                    first = Some(c);
                }
                last = Some(c);
            }
        }
        let mut current = String::new();
        current.push(first.unwrap());
        current.push(last.unwrap());
        res += current.parse::<i32>().unwrap();
    }
    format!("{res}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
