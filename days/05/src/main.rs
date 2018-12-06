use std::collections::VecDeque;

// Walk the polymer string, comparing diagraphs to find polarity shifts and remove them.
fn react_polymer(polymer: &[u8]) -> VecDeque<u8> {
    let mut q = Vec::from(polymer);

    // Stack of "units" build up in `acc`
    let mut acc = VecDeque::new();
    acc.push_back(q.pop().unwrap());

    while let Some(a) = q.pop() {

        /* Conditional avoids nested if blocks, to satisfy BC involving `acc` */

        let mut is_dupe = false;

        if let Some(b) = acc.back() {
            // 'a' XOR 'A' is 32, which holds for all letters
            is_dupe = a ^ b == 32
        }

        if is_dupe {
            acc.pop_back();
        } else {
            acc.push_back(a);
        }
    }
    acc
}

// "How many units remain after fully reacting the polymer you scanned?"
fn part1(puzzle: &[u8]) -> usize {
    react_polymer(puzzle).len()
}

// "What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?"
fn part2(puzzle: &[u8]) -> usize {
    // Do a first pass - which trims the string down significantly.
    let puzzle = react_polymer(puzzle);

    let mut shortest = usize::max_value();

    // For such long input, we can just assume every character is present
    // somewhere and brute force attempt each as a candidate.
    for ch in b'a'..=b'z' {
        let trimmed: Vec<u8> = puzzle.iter()
            .filter(|&&c| c.to_ascii_lowercase() != ch)
            .map(|c| *c)
            .collect();

        let size = react_polymer(&trimmed).len();
        if size < shortest {
            shortest = size;
        }
    }
    shortest
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &[u8] = b"dabAcCaCBAcCcaDA";
    #[test]
    fn example_1() {
        let expect = 10; // Ten remaining units in the polymer.
        assert_eq!(part1(&TEST), expect);
    }

    #[test]
    fn example_2() {
        // Removing all the 'c/C' instances and reacting the polymer leaves 4 units.
        let expect = 4;
        assert_eq!(part2(&TEST), expect);
    }
}

fn main() {
    let input = include_str!("../input");

    // NEWLINE, MY ARCH NEMESIS!
    let input = input.trim_end().as_bytes();

    println!("part1: {}", part1(&input[..]));
    println!("part2: {}", part2(&input[..]));
}
