use std::collections::HashSet;

fn parse(s: &str) -> Vec<i64> {
    s.split(|c| c == ',' || c == ' ' || c == '\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn freq(nums: Vec<i64>) -> i64 { nums.iter().sum() }

fn part1(s: &str) -> i64 {
    freq(parse(s))
}

fn part2(s: &str) -> i64 {
    let frequencies = parse(s);
    //println!("Frequencies: {:?}", frequencies);

    let mut cur = 0;
    let mut seen = HashSet::<i64>::new();
    seen.insert(cur); // Not really a better way to initialize a map with default values
    frequencies.iter().cycle().find_map(|n| {
        cur += n;
        seen.replace(cur)
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let tests = vec!(
            ("+1, +1, +1",   3),
            ("+1, +1, -2",   0),
            ("-1, -2, -3",  -6));
        for tt in tests {
            assert_eq!(part1(tt.0), tt.1);
        }
    }

    #[test]
    fn example_2() {
        let tests = vec!(
            ("+1, -1",  0),
            ("+3, +3, +4, -2, -4",  10),
            ("-6, +3, +8, +5, -6",  5),
            ("+7, +7, -2, -7, -4",  14));
        for tt in tests {
            assert_eq!(part2(tt.0), tt.1);
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
