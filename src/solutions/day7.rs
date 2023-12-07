use std::cmp::Ordering;

use bstr::ByteSlice;

use crate::utils::parsing::ByteParsing;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn has_count(count: u8, counts: &[u8]) -> bool {
    counts.iter().any(|&c| c == count)
}

fn get_kind(hand: &(u32, [u8; 5]), jokers: bool) -> HandKind {
    const POSSIBLE_BYTES: &[u8; 13] = b"JAKQT98765432";
    const CARD_COUNT: usize = POSSIBLE_BYTES.len();
    let mut counts = [0; CARD_COUNT];
    for byte in hand.1 {
        counts[POSSIBLE_BYTES.find_byte(byte).unwrap()] += 1;
    }

    if !jokers || counts[0] == 0 {
        if counts.iter().any(|&c| c == 5) {
            HandKind::FiveOfAKind
        } else if counts.iter().any(|&c| c == 4) {
            HandKind::FourOfAKind
        } else if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
            HandKind::FullHouse
        } else if counts.iter().any(|&c| c == 3) {
            HandKind::ThreeOfAKind
        } else if counts.iter().filter(|&c| c == &2).count() == 2 {
            HandKind::TwoPairs
        } else if counts.iter().any(|&c| c == 2) {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    } else {
        let joker_count = counts[0];
        let counts: &[u8; 12] = counts[1..].try_into().unwrap();
        if has_count(4, counts)
            || has_count(3, counts) && joker_count == 2
            || has_count(2, counts) && joker_count == 3
            || has_count(1, counts) && joker_count == 4
            || joker_count == 5
        {
            HandKind::FiveOfAKind
        } else if has_count(3, counts)
            || has_count(2, counts) && joker_count == 2
            || has_count(1, counts) && joker_count == 3
        {
            HandKind::FourOfAKind
        } else if has_count(3, counts) && has_count(2, counts)
            || counts.iter().filter(|&c| c == &2).count() == 2
        {
            HandKind::FullHouse
        } else if has_count(2, counts) || has_count(1, counts) && joker_count == 2 {
            HandKind::ThreeOfAKind
        } else {
            HandKind::OnePair
        }
    }
}

fn compare_hands(a: &(u32, [u8; 5]), b: &(u32, [u8; 5]), jokers: bool) -> Ordering {
    const POSSIBLE_BYTES: &[u8; 13] = b"AKQJT98765432";
    let kind_a = get_kind(a, jokers);
    let kind_b = get_kind(b, jokers);
    if kind_a < kind_b {
        return Ordering::Greater;
    } else if kind_a > kind_b {
        return Ordering::Less;
    } else {
        for (b_a, b_b) in a.1.iter().zip(b.1.iter()) {
            if b_a != b_b {
                if jokers {
                    if b_a == &b'J' {
                        return Ordering::Less;
                    } else if b_b == &b'J' {
                        return Ordering::Greater;
                    }
                }
                if POSSIBLE_BYTES.find_byte(*b_a) < POSSIBLE_BYTES.find_byte(*b_b) {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }
    }
    Ordering::Equal
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut hands: Vec<(u32, [u8; 5])> = Vec::new();
    for line in input.lines() {
        hands.push((line[6..].as_num::<u32>(), line[0..5].try_into().unwrap()));
    }
    hands.sort_by(|a, b| compare_hands(a, b, false));
    let mut score = 0;
    for (multiply, hand) in hands.iter().enumerate() {
        score += ((multiply + 1) as u32) * hand.0;
    }
    score
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = input.as_bytes();
    let mut hands: Vec<(u32, [u8; 5])> = Vec::new();
    for line in input.lines() {
        hands.push((line[6..].as_num::<u32>(), line[0..5].try_into().unwrap()));
    }
    hands.sort_by(|a, b| compare_hands(a, b, true));
    let mut score = 0;
    for (multiply, hand) in hands.iter().enumerate() {
        score += ((multiply + 1) as u32) * hand.0;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const INPUT2: &str = INPUT1;

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("6440"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("5905"))
    }
}

