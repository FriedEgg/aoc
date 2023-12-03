use std::time::Duration;

fn main() {
    let solutions: Vec<&dyn Fn() -> Duration> = vec![
        &aoc::day::_1::solve,
    ];

    for (i, solution) in solutions.iter().enumerate() {
        println!(
            "Day {:02}",
            i + 1,
        );
        let t = solution();
        println!(
            "\tElapsed: {}ms",
            format!("{:.4}", t.as_nanos() as f64 / 1_000_000.0),
        )
    }
}