use crate::Solution;

pub struct _2;

impl Solution for _2 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn part_a(&self, input: &str) -> usize {
        let mut sum = 0;
        for line in input.lines().into_iter() {
            let (game_id, game) = line.trim_start_matches("Game ").split_once(':').unwrap();
            let mut possible = true;
            for test in game.split([';', ',']) {
                let (n, color) = test.trim().split_once(' ').unwrap();
                let n: usize = n.parse().unwrap();
                match color {
                    "red" => {possible &= n <= 12},
                    "green" => {possible &= n <= 13},
                    "blue" => {possible &= n <= 14},
                    _ => unreachable!(),
                }
            }
            sum += if possible {game_id.parse().unwrap()} else {0};
        };
        sum
    }

    fn part_b(&self, input: &str) -> usize {
        let mut sum = 0;
        for line in input.lines().into_iter() {
            let game = line.trim_start_matches("Game ").split_once(':').unwrap().1;
            let (mut r, mut g, mut b) = (0, 0, 0);
            for test in game.split([';', ',']) {
                let (n, color) = test.trim().split_once(' ').unwrap();
                let n: usize = n.parse().unwrap();
                match color {
                    "red" => {r = r.max(n)},
                    "green" => {g = g.max(n)},
                    "blue" => {b = b.max(n)},
                    _ => unreachable!(),
                }
            }
            sum += r * g * b;
        };
        sum
    }    
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::Solution;
    use super::_2;

    const CALIBRATION: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_part_a() {
        assert_eq!(_2.part_a(&CALIBRATION), 8);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(_2.part_b(&CALIBRATION), 2286);
    }
}
