use crate::aoc_helpers::grid::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut distance = 0;
    let grid = Grid::from_str(input, |((_, _), c)| c as u8);
    let ((x, y), _) = grid.iter().find(|&((_, _), c)| *c == b'S').unwrap();
    let mut x = x as isize;
    let mut y = y as isize;
    let mut dx = 0;
    let mut dy = 0;
    if matches!(grid.get_at_i((x - 1, y)), Some(b'-' | b'L' | b'F')) {
        dx = -1;
    } else if matches!(grid.get_at_i((x + 1, y)), Some(b'-' | b'J' | b'7')) {
        dx = 1;
    } else if matches!(grid.get_at_i((x, y - 1)), Some(b'|' | b'L' | b'J')) {
        dy = -1;
    } else if matches!(grid.get_at_i((x, y + 1)), Some(b'|' | b'F' | b'7')) {
        dy = 1;
    } else {
        panic!("Invalid input?");
    }
    loop {
        distance += 1;
        (x, y) = (x + dx, y + dy);
        (dx, dy) = match grid[(x, y)] {
            b'-' => (dx, dy),
            b'|' => (dx, dy),
            b'L' => (dy, dx),
            b'7' => (dy, dx),
            b'J' => (-dy, -dx),
            b'F' => (-dy, -dx),
            b'S' => {
                break;
            }
            _ => panic!("invalid"),
        };
    }
    distance / 2
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    const INPUT2: &str = INPUT1;

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT1).to_string(), String::from("4"))
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT2).to_string(), String::from("0"))
    }
}
