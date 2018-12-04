use std::collections::{HashMap, HashSet};

fn parse_line(cap: regex::Captures) -> Vec<u16> {
    cap.iter().skip(1)
        .filter_map(|n| n.unwrap().as_str().parse::<u16>().ok())
        .collect()
}

// "How many square inches of fabric are within two or more claims?"
fn part1(puzzle: &str) -> usize {
    // Sample match: #12 @ 369,930: 21x14
    let re = regex::Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)\n?").unwrap();

    // a sparse matrix from x,y position to claim count (overlap)
    let mut grid = HashMap::<(u16, u16), u8>::new();

    for cap in re.captures_iter(puzzle) {
        let nums = parse_line(cap);
        let (x,y,w,h) = (nums[0], nums[1], nums[2], nums[3]);
        for i in y..y+h {
            for j in x..x+w {
                *grid.entry((j,i)).or_insert(0) += 1;
            }
        }
    }

    // Now simply count all the spots where there are more than one claim.
    grid.values().filter(|&&n| n > 1).count()
}

// "What is the ID of the only claim that doesn't overlap?"
fn part2(puzzle: &str) -> u16 {
    let re = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\n?").unwrap();

    // Simply extend the same sparse map solution, and brute force the answer
    // (many wasted cycles! Could do better with quad trees as a step 1)
    let mut grid = HashMap::<(u16,u16), Vec<u16>>::new();
    let mut ids = HashSet::<u16>::new();

    for cap in re.captures_iter(puzzle) {
        let nums = parse_line(cap);
        let (id,x,y,w,h) = (nums[0], nums[1], nums[2], nums[3], nums[4]);
        ids.insert(id);

        for y in y..y+h {
            for x in x..x+w {
                // This time, record all the ids at each point
                grid.entry((x,y))
                    .or_insert(Vec::new())
                    .push(id);
            }
        }
    }

    // Across the grid, remove all ids that ever collide on a point (their 'claims')
    let colliders = grid.values().filter(|ids| ids.len() > 1).flatten().cloned().collect();
    ids = ids.difference(&colliders).cloned().collect();

    // Afterwards, there must be one ID remaining, which never had a collision.
    assert_eq!(ids.len(), 1);
    ids.into_iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = " \
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn example_1() {
        let expect = 2 * 2; // Center overlap, claimed by #1 and #2
        assert_eq!(part1(&TEST), expect);
    }

    #[test]
    fn example_2() {
        let expect = 3; // Only claim #3 has no overlap
        assert_eq!(part2(&TEST), expect);
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
