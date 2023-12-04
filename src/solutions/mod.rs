use std::fmt::Display;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub fn get_bench_solutions() -> Vec<[Box<dyn Fn(&str) -> std::time::Duration>; 2]> {
    fn get_bench<D: Display + 'static>(
        function: fn(&str) -> D,
    ) -> Box<dyn Fn(&str) -> std::time::Duration> {
        Box::new(move |input| {
            let start = std::time::Instant::now();
            let _ = function(input);
            start.elapsed()
        })
    }
    Vec::from([
        [get_bench(day1::part_1), get_bench(day1::part_2)],
        [get_bench(day2::part_1), get_bench(day2::part_2)],
        [get_bench(day3::part_1), get_bench(day3::part_2)],
        [get_bench(day4::part_1), get_bench(day4::part_2)],
        [get_bench(day5::part_1), get_bench(day5::part_2)],
        [get_bench(day6::part_1), get_bench(day6::part_2)],
        [get_bench(day7::part_1), get_bench(day7::part_2)],
        [get_bench(day8::part_1), get_bench(day8::part_2)],
        [get_bench(day9::part_1), get_bench(day9::part_2)],
        [get_bench(day10::part_1), get_bench(day10::part_2)],
        [get_bench(day11::part_1), get_bench(day11::part_2)],
        [get_bench(day12::part_1), get_bench(day12::part_2)],
        [get_bench(day13::part_1), get_bench(day13::part_2)],
        [get_bench(day14::part_1), get_bench(day14::part_2)],
        [get_bench(day15::part_1), get_bench(day15::part_2)],
        [get_bench(day16::part_1), get_bench(day16::part_2)],
        [get_bench(day17::part_1), get_bench(day17::part_2)],
        [get_bench(day18::part_1), get_bench(day18::part_2)],
        [get_bench(day19::part_1), get_bench(day19::part_2)],
        [get_bench(day20::part_1), get_bench(day20::part_2)],
        [get_bench(day21::part_1), get_bench(day21::part_2)],
        [get_bench(day22::part_1), get_bench(day22::part_2)],
        [get_bench(day23::part_1), get_bench(day23::part_2)],
        [get_bench(day24::part_1), get_bench(day24::part_2)],
        [get_bench(day25::part_1), get_bench(day25::part_2)],
    ])
}

pub fn get_solutions() -> Vec<[Box<dyn Fn(&str) -> String>; 2]> {
    fn get_string<D: Display + 'static>(function: fn(&str) -> D) -> Box<dyn Fn(&str) -> String> {
        Box::new(move |input| function(input).to_string())
    }
    Vec::from([
        [get_string(day1::part_1), get_string(day1::part_2)],
        [get_string(day2::part_1), get_string(day2::part_2)],
        [get_string(day3::part_1), get_string(day3::part_2)],
        [get_string(day4::part_1), get_string(day4::part_2)],
        [get_string(day5::part_1), get_string(day5::part_2)],
        [get_string(day6::part_1), get_string(day6::part_2)],
        [get_string(day7::part_1), get_string(day7::part_2)],
        [get_string(day8::part_1), get_string(day8::part_2)],
        [get_string(day9::part_1), get_string(day9::part_2)],
        [get_string(day10::part_1), get_string(day10::part_2)],
        [get_string(day11::part_1), get_string(day11::part_2)],
        [get_string(day12::part_1), get_string(day12::part_2)],
        [get_string(day13::part_1), get_string(day13::part_2)],
        [get_string(day14::part_1), get_string(day14::part_2)],
        [get_string(day15::part_1), get_string(day15::part_2)],
        [get_string(day16::part_1), get_string(day16::part_2)],
        [get_string(day17::part_1), get_string(day17::part_2)],
        [get_string(day18::part_1), get_string(day18::part_2)],
        [get_string(day19::part_1), get_string(day19::part_2)],
        [get_string(day20::part_1), get_string(day20::part_2)],
        [get_string(day21::part_1), get_string(day21::part_2)],
        [get_string(day22::part_1), get_string(day22::part_2)],
        [get_string(day23::part_1), get_string(day23::part_2)],
        [get_string(day24::part_1), get_string(day24::part_2)],
        [get_string(day25::part_1), get_string(day25::part_2)],
    ])
}
