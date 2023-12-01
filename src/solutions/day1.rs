use bstr::ByteSlice;
use std::fmt::Display;

pub fn part_1(input: &str) -> impl Display {
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    const OFFSET_PER_LINE: u32 = 11 * b'0' as u32;
    let mut line_count = 0;
    for line in input.as_bytes().lines() {
        line_count += 1;
        let mut line_iter = line.iter();
        let first = line_iter.find(|c| *c <= &b'9').unwrap();
        sum2 += *line_iter.rfind(|c| *c <= &b'9').unwrap_or(first) as u32;
        sum1 += *first as u32;
    }
    sum1 * 10 + sum2 - line_count * OFFSET_PER_LINE
}
pub fn part_2(input: &str) -> impl Display {
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
        .sum::<u32>()
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
        assert_eq!(part_1(INPUT).to_string(), String::from("142"))
    }
    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("281"))
    }
}
