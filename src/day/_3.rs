use std::collections::HashMap;

use crate::Solution;

pub struct _3;

impl Solution for _3 {
    fn name(&self) -> &'static str {
        "Gear Ratios"
    }

    fn part_a(&self, input: &str) -> usize {
        let mut sum = 0;
        let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        for (r, line) in lines.iter().enumerate() {
            let mut c = 0;
            while c < line.len() {
                let start = c;
                let mut symbol: Option<_> = None;
                while c < line.len() && line[c].is_ascii_digit() {
                    symbol = symbol.or_else(|| search(&lines, r, c));
                    c += 1;
                }
                if symbol.is_some() {
                    let part = std::str::from_utf8(&line[start..c]).unwrap().parse::<usize>().unwrap();
                    //println!("{}: {} found {}",r,std::str::from_utf8(line).unwrap(),gear)
                    sum += part;
                }
                c += 1;
            }
        }
        sum
    }

    fn part_b(&self, input: &str) -> usize {
        let mut gears = HashMap::new();
        let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        for (r, line) in lines.iter().enumerate() {
            let mut c = 0;
            while c < line.len() {
                let start = c;
                let mut symbol: Option<_> = None;
                while c < line.len() && line[c].is_ascii_digit() {
                    symbol = symbol.or_else(|| search(&lines, r, c));
                    c += 1;
                }
                if symbol.is_some() {
                    let part = std::str::from_utf8(&line[start..c]).unwrap().parse::<usize>().unwrap();
                    //println!("{}: {} found {}",r,std::str::from_utf8(line).unwrap(),gear)
                    gears.entry(symbol.unwrap()).or_insert(Vec::new()).push(part)
                }
                c += 1;
            }
        }
        gears.iter()
            .filter(|(&(_,_,s),v)| s == b'*' && v.len() ==2)
            .map(|(_,v)| v[0] * v[1])
            .sum()
    }
}

fn search(lines: &[&[u8]], r: usize, c: usize) -> Option<(usize, usize, u8)> {
    for (ro, co) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
        let _r = (r as i32 + ro) as usize;
        let _c = (c as i32 + co) as usize;
        let Some(&s) = lines.get(_r).and_then(|l| l.get(_c)) else {continue};
        if s != b'.' && !s.is_ascii_digit() {
            return Some((_r, _c, s));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::Solution;
    use super::_3;

    const CALIBRATION: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_part_a() {
        assert_eq!(_3.part_a(&CALIBRATION), 4361);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(_3.part_b(&CALIBRATION), 467835);
    }
}