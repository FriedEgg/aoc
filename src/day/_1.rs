use std::time::{Duration, Instant};

const DIGITS: &[&[u8]; 9] = &[b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine"];

pub fn solve() -> Duration {
    let s = Instant::now();

    let input = std::fs::read_to_string("input/_1.txt").unwrap();

    let part_a = part_a(&input);
    let part_b = part_b(&input);

    println!("\tPart 1: {}", part_a);
    println!("\tPart 2: {}", part_b);

    s.elapsed()
}

fn part_a(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines().map(str::as_bytes) {
        let mut digits = (0..line.len()).filter_map(|i| match line[i] {
            b'0'..=b'9' => Some((line[i] - b'0') as usize),
            _ => None,
        });
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        sum += first * 10 + last;
    }
    sum
}

fn part_b(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines().map(str::as_bytes) {
        let mut digits = (0..line.len()).filter_map(|i| match line[i] {
            b'0'..=b'9' => Some((line[i] - b'0') as usize),
            _ => DIGITS.iter()
                .enumerate()
                .find_map(|(di, d)| line[i..].starts_with(d).then_some(di + 1)),
        });
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::part_a;
    use super::part_b;

    const CALIBRATION_A: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const CALIBRATION_B: &str = indoc! {"
        two1nine
        eighttwothree
        abcone2threexyz
        xtwoone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&CALIBRATION_A), 142);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&CALIBRATION_B), 281);
    }
}