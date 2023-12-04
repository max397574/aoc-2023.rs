use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input.as_bytes().lines() {
        let mut line_score = 0;
        let mut getting_winning_numbers = true;
        let mut winning_numbers = Vec::new();
        for block in line.split(|&byte| byte == b' ') {
            if block.len() == 0 {
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
            } else {
                if winning_numbers.contains(&block) {
                    if line_score == 0 {
                        line_score = 1;
                    } else {
                        line_score *= 2;
                    }
                }
            }
        }
        score += line_score;
    }
    score
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    0
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
    const INPUT2: &str = "";

    #[test]
    fn _part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("13"))
    }

    #[test]
    fn _part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("0"))
    }
}
