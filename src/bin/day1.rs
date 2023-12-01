use bstr::ByteSlice;

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.as_bytes().lines() {
        let mut line_iter = line.iter();
        let first = line_iter.find(|c| *c <= &b'9').unwrap();
        let last = line_iter.rfind(|c| *c <= &b'9').unwrap_or(first) - b'0';
        sum += ((first - b'0') * 10 + last) as u32
    }
    sum
}
fn part_2(input: &str) -> u32 {
    let options: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let first: u32 = line
                .char_indices()
                .find_map(|(idx, char)| {
                    match char {
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            return Some(char as u32 - 48);
                        }
                        _ => {}
                    }
                    for (index, option) in options.iter().enumerate() {
                        if char == option.chars().next().unwrap() && line[idx..].starts_with(option)
                        {
                            return Some((index + 1) as u32);
                        }
                    }
                    None
                })
                .unwrap();
            let last: u32 = line
                .char_indices()
                .rev()
                .find_map(|(idx, char)| {
                    match char {
                        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            return Some(char as u32 - 48);
                        }
                        _ => {}
                    }
                    for (index, option) in options.iter().enumerate() {
                        if char == option.chars().last().unwrap() && line[..=idx].ends_with(option)
                        {
                            return Some((index + 1) as u32);
                        }
                    }
                    None
                })
                .unwrap();
            first * 10 + last
        })
        .sum()
}
fn main() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let start = std::time::Instant::now();
    println!("part1: {}", part_1(&input));
    println!("time (part1): {:?}", start.elapsed());
    let start = std::time::Instant::now();
    println!("part2: {}", part_2(&input));
    println!("time (part2): {:?}", start.elapsed());
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 142)
    }
    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2), 281)
    }
}
