fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let mut res = 0;
    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
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
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
