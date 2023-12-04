use atoi::atoi;
use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input.as_bytes().lines() {
        let mut winning_numbers: u128 = 0;
        let mut numbers_you_have: u128 = 0;
        let mut getting_winning_numbers = true;
        for block in line.split(|&byte| byte == b' ') {
            if block.is_empty() {
                continue;
            }
            if block[0] == b'C' {
                continue;
            }
            if block.len() == 2 && block[1] == b':' {
                continue;
            }
            if block[0] == b'|' {
                getting_winning_numbers = false;
                continue;
            }
            if getting_winning_numbers {
                winning_numbers |= 1 << atoi::<u8>(block).unwrap();
            } else {
                numbers_you_have |= 1 << atoi::<u8>(block).unwrap();
            }
        }
        let matches = (winning_numbers & numbers_you_have).count_ones();
        if matches != 0 {
            score += 1 << (matches - 1);
        }
    }
    score
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut matching_numbers = Vec::new();
    for line in input.as_bytes().lines() {
        let mut line_score = 0;
        let mut getting_winning_numbers = true;
        let mut winning_numbers = Vec::new();
        for block in line.split(|&byte| byte == b' ') {
            if block.is_empty() {
                continue;
            }
            if block[0] == b'C' {
                continue;
            }
            if block.len() == 2 && block[1] == b':' {
                continue;
            }
            if block[0] == b'|' {
                getting_winning_numbers = false;
                continue;
            }
            if getting_winning_numbers {
                winning_numbers.push(block);
            } else if winning_numbers.contains(&block) {
                line_score += 1;
            }
        }
        matching_numbers.push(line_score);
    }
    let mut copies = vec![1; matching_numbers.len()];
    for (idx, match_count) in matching_numbers.iter().enumerate() {
        for i in 1..=*match_count {
            copies[idx + i] += copies[idx];
        }
    }
    let mut count = 0;
    copies.iter().for_each(|x| count += x);
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const INPUT2: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("13"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("30"))
    }
}
