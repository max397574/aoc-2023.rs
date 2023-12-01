use std::{
    env::args,
    time::{Duration, Instant},
};

mod solutions;

pub fn main() {
    let mut args = args();
    let _binary_name = args.next().unwrap();
    let day: usize = args.next().unwrap().parse().unwrap();
    let part: usize = args.next().unwrap().parse().unwrap();
    let bench = args.any(|arg| arg == "--bench");

    let input = std::fs::read_to_string(format!("input/day{}.txt", day)).unwrap();

    let solution = &solutions::get_solutions()[day - 1][part - 1];

    if bench {
        let mut total = Duration::ZERO;
        const WARMUPS: u8 = 3;
        const SECONDS: u64 = 5;
        let duration = Duration::from_secs(SECONDS);
        for _ in 0..WARMUPS {
            let _ = solution(&input);
        }
        let first_start = Instant::now();
        let mut runs = 0;
        while first_start.elapsed() < duration {
            let start = Instant::now();
            let _ = solution(&input);
            total += start.elapsed();
            runs += 1;
        }
        println!(
            "Average time with {WARMUPS} warmup runs and running over {SECONDS} seconds ({runs} runs): {:?}",
            total / runs
        );
        return;
    }

    let start = std::time::Instant::now();
    let result = solution(&input);
    println!("Duration: {:?}", start.elapsed());
    println!("Result: {result:?}");
}
