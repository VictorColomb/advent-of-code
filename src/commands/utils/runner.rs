use std::{cmp::min, fmt::Display, hint::black_box, time::Instant};

use super::Day;

/// Run a solution part and print the result to stdout.
///
/// In release mode, the solution part will be benchmarked and the result will be printed as well.
/// The benchmark will be run for 5 seconds or 10k iterations, whichever is the smallest.
#[allow(dead_code)]
pub fn run_part<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    #[cfg(debug_assertions)]
    run_debug(func, input, day, part);

    #[cfg(not(debug_assertions))]
    run_release(func, input, day, part);
}

#[cfg(debug_assertions)]
fn run_debug<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    let result = func(input.clone());

    match result {
        Some(result) => {
            println!("🎄 Day {} Part {}: {}", day, part, result);
        }
        None => println!("🎄 Day {} Part {}: None", day, part),
    }
}

#[allow(dead_code)]
fn run_release<I: Clone, T: Display>(func: impl Fn(I) -> Option<T>, input: I, day: Day, part: u8) {
    let cloned = input.clone();
    let start = Instant::now();
    let result = func(cloned);
    let base_time = start.elapsed();

    let result_str = match result {
        Some(result) => {
            format!("🎄 Day {} Part {}: {}", day, part, result)
        }
        None => format!("🎄 Day {} Part {}: None", day, part),
    };
    print!("{} | Benching...", result_str);

    let nb_iter = min(10000, 5_000_000_000 / base_time.as_nanos() as usize);
    let mut total_time = 0;
    for _ in 0..nb_iter {
        let cloned = input.clone();
        let start = Instant::now();
        black_box(func(black_box(cloned)));
        total_time += start.elapsed().as_nanos() as usize;
    }

    let duration = std::time::Duration::from_nanos((total_time / nb_iter) as u64);
    println!(
        "\x1b[2K\r{} ({:.1?} @ {} samples)",
        result_str, duration, nb_iter
    );
}
