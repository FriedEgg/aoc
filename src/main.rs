use std::time::Instant;

use aoc::Solution;

fn main() {
    let solutions: Vec<&dyn Solution> = vec![
        &aoc::day::__1,
        &aoc::day::__2,
    ];

    for (i, solution) in solutions.iter().enumerate() {
        println!(
            "--- Day {:02} {} ---",
            i + 1,
            solution.name(),
        );

        let start = Instant::now();

        let input = std::fs::read_to_string(format!("input/_{}.txt", i + 1)).unwrap();

        for part in 'A'..='B' {
            let s = Instant::now();
            let a = match part {
                'A' => solution.part_a(&input),
                'B' => solution.part_b(&input),
                _ => unreachable!(),
            };
            let t = s.elapsed();
            println!(
                "\t Part {}: {}, Duration: {}ms",
                part,
                a,
                format!("{:.4}", t.as_nanos() as f64 / 1_000_000.0),
            );
        }

        println!(
            "\t Total Duration: {}ms",
            format!("{:.4}", start.elapsed().as_nanos() as f64 / 1_000_000.0),
        );

    }
}