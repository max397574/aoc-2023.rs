use bstr::ByteSlice;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    const RED_BYTES: &[u8] = b"red";
    const GREEN_BYTES: &[u8] = b"green";
    const BLUE_BYTES: &[u8] = b"blue";
    let mut sum = 0;
    let mut id = 0;
    for line in input.as_bytes().lines() {
        id += 1;
        let mut id_cube_split = line.split_str(":");
        id_cube_split.next();
        let mut valid = true;
        for cubes in id_cube_split
            .next()
            .unwrap()
            .split(|&char| char == b';' || char == b',')
        {
            let mut split = cubes.trim().split_str(" ");
            let num: u32 = split
                .next()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            valid = valid
                && match split.next().unwrap() {
                    RED_BYTES => num <= MAX_RED,
                    GREEN_BYTES => num <= MAX_GREEN,
                    BLUE_BYTES => num <= MAX_BLUE,
                    _ => unreachable!(),
                }
        }
        if valid {
            sum += id;
        }
    }
    sum
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut max_red: u32;
    let mut max_green: u32;
    let mut max_blue: u32;
    const RED_BYTES: &[u8] = b"red";
    const GREEN_BYTES: &[u8] = b"green";
    const BLUE_BYTES: &[u8] = b"blue";
    let mut sum = 0;
    for line in input.as_bytes().lines() {
        max_red = 0;
        max_green = 0;
        max_blue = 0;
        let mut id_cube_split = line.split_str(":");
        id_cube_split.next();
        for cubes in id_cube_split
            .next()
            .unwrap()
            .split(|&char| char == b';' || char == b',')
        {
            let mut split = cubes.trim().split_str(" ");
            let num: u32 = split
                .next()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            match split.next().unwrap() {
                RED_BYTES => max_red = max_red.max(num),
                GREEN_BYTES => max_green = max_green.max(num),
                BLUE_BYTES => max_blue = max_blue.max(num),
                _ => unreachable!(),
            }
        }
        sum += max_green * max_red * max_blue;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    const _INPUT2: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() {
        assert_eq!(part_1(_INPUT1).to_string(), String::from("8"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(_INPUT2).to_string(), String::from("2286"))
    }
}
