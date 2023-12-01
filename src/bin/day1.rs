fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap() as u32 - 48;
            let last = line.chars().rfind(|c| c.is_ascii_digit()).unwrap() as u32 - 48;
            first * 10 + last
        })
        .sum()
}
fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first: u32 = line
                .char_indices()
                .find_map(|(idx, char)| match char {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(char as u32 - 48),
                    'o' => {
                        if line[idx..].starts_with("one") {
                            Some(1)
                        } else {
                            None
                        }
                    }
                    't' => {
                        if line[idx..].starts_with("two") {
                            Some(2)
                        } else if line[idx..].starts_with("three") {
                            Some(3)
                        } else {
                            None
                        }
                    }
                    'f' => {
                        if line[idx..].starts_with("four") {
                            Some(4)
                        } else if line[idx..].starts_with("five") {
                            Some(5)
                        } else {
                            None
                        }
                    }
                    's' => {
                        if line[idx..].starts_with("six") {
                            Some(6)
                        } else if line[idx..].starts_with("seven") {
                            Some(7)
                        } else {
                            None
                        }
                    }
                    'e' => {
                        if line[idx..].starts_with("eight") {
                            Some(8)
                        } else {
                            None
                        }
                    }
                    'n' => {
                        if line[idx..].starts_with("nine") {
                            Some(9)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .unwrap();
            let last: u32 = line
                .char_indices()
                .rev()
                .find_map(|(idx, char)| match char {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(char as u32 - 48),
                    'e' => {
                        if line[..=idx].ends_with("one") {
                            Some(1)
                        } else if line[..=idx].ends_with("three") {
                            Some(3)
                        } else if line[..=idx].ends_with("five") {
                            Some(5)
                        } else if line[..=idx].ends_with("nine") {
                            Some(9)
                        } else {
                            None
                        }
                    }
                    'o' => {
                        if line[..=idx].ends_with("two") {
                            Some(2)
                        } else {
                            None
                        }
                    }
                    'r' => {
                        if line[..=idx].ends_with("four") {
                            Some(4)
                        } else {
                            None
                        }
                    }
                    'x' => {
                        if line[..=idx].ends_with("six") {
                            Some(6)
                        } else {
                            None
                        }
                    }
                    'n' => {
                        if line[..=idx].ends_with("seven") {
                            Some(7)
                        } else {
                            None
                        }
                    }
                    't' => {
                        if line[..=idx].ends_with("eight") {
                            Some(8)
                        } else {
                            None
                        }
                    }
                    _ => None,
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
