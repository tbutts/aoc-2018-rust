use std::collections::HashMap;
use std::collections::HashSet;

fn part1(puzzle: &str) -> u32 {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;
    // rewrite as fold?
    for line in puzzle.lines() {
        let mut letters = HashMap::<char, u8>::new();
        for c in line.chars() {
            let e = letters.entry(c).or_insert(0);
            *e += 1;
        }

        let counts: HashSet<u8> = letters.values().cloned().collect();
        if counts.contains(&2) { twos += 1; }
        if counts.contains(&3) { threes += 1; }
    }

    let checksum = twos * threes;
    checksum
}

fn part2(puzzle: &str) -> String {
    let mut ids: Vec<&str> = puzzle.split_whitespace().collect();

    let mut next = ids.pop();
    while next.is_some() {
        let id = next.unwrap();
        for other in ids.iter() {
            // Save offending character position
            let mut maybe_needle: usize = 0;

            assert!(id.len() == other.len());

            // Track differences in strings, break early if more than 1 is a problem
            let mut diff: u8 = 0;
            for (index, (a, b)) in id.chars().zip(other.chars()).enumerate() {
                if a != b {
                    maybe_needle = index;
                    diff += 1;
                    if diff > 1 {
                        break
                    }
                }
            }

            if diff == 1 {
                let pos = maybe_needle;
                return [&id[0..pos], &id[pos+1..]].concat();
            }
        }
        next = ids.pop();
    }
    panic!("Unable to find ids with one character difference!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let test = vec!("abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab")
            .join("\n");
        let expect = 4 * 3;
        assert_eq!(part1(&test), expect);
    }

    #[test]
    fn example_2() {
        let test = vec!("abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz")
            .join("\n");
        let expect = "fgij";
        assert_eq!(part2(&test), expect);
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
