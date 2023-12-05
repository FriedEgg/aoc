use crate::Solution;

pub struct _4;

impl Solution for _4 {
    fn name(&self) -> &'static str {
        "Scratchcards"
    }

    fn part_a(&self, input: &str) -> usize {
        let mut sum = 0;
        for mut line in input.lines() {
            line = line.split_once(':').unwrap().1;
            let (wins, wants) = line.split_once('|').unwrap();
            let wins = wins.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let won = wants.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .filter(|w| wins.contains(w))
                .count() as u32;
            if won > 0 {
                sum += 2usize.pow(won - 1)
            }
        }
        sum
    }

    fn part_b(&self, input: &str) -> usize {
        let mut copies = vec![1; input.lines().count()];
        for (i, mut line) in input.lines().enumerate() {
            line = line.split_once(':').unwrap().1;
            let (wins, wants) = line.split_once('|').unwrap();
            let wins = wins.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let won = wants.split_whitespace()
                .map(|w| w.parse::<usize>().unwrap())
                .filter(|w| wins.contains(w))
                .count();
            for j in 0..won {
                copies[i+j+1] += copies[i];
            }
        }
        copies.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::Solution;
    use super::_4;

    const CALIBRATION: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_part_a() {
        assert_eq!(_4.part_a(&CALIBRATION), 13);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(_4.part_b(&CALIBRATION), 30);
    }
}