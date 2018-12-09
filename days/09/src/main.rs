/* Elves are dumb */

use std::collections::VecDeque;

type Marble = u32;

/* VecDeque is fantastic when items are placed on either end of the queue.
   BUT NOTE: Can't use standard rotate_* methods on a VecDeque. */
type Game = VecDeque<u32>;

fn rotate_clockwise(game: &mut Game) {
    let rot = game.pop_front().unwrap();
    game.push_back(rot);
}

fn rotate_counter_clockwise(game: &mut Game) {
    let rot = game.pop_back().unwrap();
    game.push_front(rot);
}

fn play(puzzle: &str, embiggened: bool) -> u32 {
    let factor = if embiggened {100} else {1};
    let (players, last_val) = {
        let nums = puzzle.split_whitespace()
            .filter_map(|snum| snum.parse().ok())
            .collect::<Vec<u32>>();
        (nums[0], nums[1]*factor)
    };
    println!("players: {}, last_val: {}", players, last_val);

    let mut scores = vec![0u32; players as usize];
    let mut current: Marble = 0;

    // Memory space caps out at the last marble placed,
    // plus one initial, and subtracting all the marbles removed & not placed every 23.
    let game = &mut VecDeque::with_capacity(
        last_val as usize+1 - (last_val as usize / 23)*2
    );
    game.push_back(current);

    // It's your old pal magic constants!
    while current < last_val {
        current += 1;
        if current % 23 != 0 {
            rotate_clockwise(game);
            game.push_back(current);
        } else {
            // *something different*
            for _ in 0..7 {
                rotate_counter_clockwise(game);
            }

            let mut score = current;
            score += game.pop_back().unwrap();

            let idx = ((current+1) % players) as usize;
            scores[idx] += score;

            rotate_clockwise(game);
        }
    }
    *scores.iter().max().unwrap()
}

// "What is the winning Elf's score?"
fn part1(puzzle: &str) -> u32 {
    play(puzzle, false)
}

// "What would the new winning Elf's score be if the number
//  of the last marble were 100 times larger?"
fn part2(puzzle: &str) -> u32 {
    play(puzzle, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let tests = vec!(
            ("10 players; last marble is worth 25 points", 32),
            ("10 players; last marble is worth 1618 points", 8317),
            ("13 players; last marble is worth 7999 points", 146373),
            ("17 players; last marble is worth 1104 points", 2764),
            ("21 players; last marble is worth 6111 points", 54718),
            ("30 players; last marble is worth 5807 points", 37305),
        );
        for (test, expect) in tests {
            assert_eq!(part1(&test), expect);
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
