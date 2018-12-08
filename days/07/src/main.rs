use std::collections::{HashMap, HashSet, BinaryHeap};

pub mod minheap; // Look for `src/minheap.rs` and add it to the include path.
// Reverse-ordered primitives (wrapped in tuples - the best rust's type system will allow)
use ::minheap::{RChar, RUint32};

struct Flags { nworkers: usize, time_offset: u32 }
struct Output { order_of_steps: String, time_taken: u32 }

// "In what order should the steps in your instructions be completed?"
fn part1(puzzle: &str) -> String {
    assemble_sleigh(puzzle, Flags{ nworkers: 1, time_offset: 0}).order_of_steps
}

// "With 5 workers and the 60+ second step durations described above,
//  how long will it take to complete all of the steps?"
fn part2(puzzle: &str, flags: Flags) -> u32 {
    assemble_sleigh(puzzle, flags).time_taken
}

/* Holy moly, producing usable iterators is funky */
fn parse_digraphs<'a>(puzzle: &'a str) -> impl Iterator<Item = (char, char)> + 'a {
    puzzle.lines()
        .map(|line| line.as_bytes())
        // Flimsy parsers make the world go round!
        .map(|line| (line[5] as char, line[36] as char))
}

// Walks the graph formed from the instructions in lexical order.
// Flags can be provided to simulate concurrent worker elves.
fn assemble_sleigh(puzzle: &str, flags: Flags) -> Output {
    let (mut edges, mut indegrees) =
        parse_digraphs(puzzle)
        // Using the 2char digraphs, build adjacency map & in-degree map
        .fold((HashMap::<char, BinaryHeap<RChar>>::new(), HashMap::<char,u8>::new()),
        |(mut edges, mut indegrees), (v,w)| {
            edges.entry(v).or_default().push(RChar(w));
            *indegrees.entry(w).or_default() += 1;
            (edges, indegrees)
        });

    // Collect all vertecies w/ no dependencies (they aren't present within `indegrees`)
    // and keep them sorted lexographically.
    let mut queue = {
        let xs = edges.keys().collect::<HashSet<_>>();
        let ys = indegrees.keys().collect::<HashSet<_>>();
        xs.difference(&ys).map(|&&c| RChar(c)).collect::<BinaryHeap<RChar>>()
    };

    // eprintln!("Edges:");
    // for (k,v) in &edges {
    //     eprintln!("{}: {:?}", k, v);
    // }
    // eprintln!("In-Degree: {:?}", indegrees);
    // eprintln!("Initial Queue: {:?}", queue);

    // Workers pull instructions off the queue, and finish them in 
    // the order: (chronological, lexographical)
    //
    // Rust's std heap is a max-heap, but by providing reverse-ordered types from
    // the standard primitives, a min-heap is fully possible. Here, it even maintains
    // the order of the components in the 2-pair - so time is higher priority than
    // lexographical sort.
    let mut workers: BinaryHeap<(RUint32, RChar)> = BinaryHeap::new();
    // As instructions are finished, they get plopped on the output queue.
    let mut out = Vec::<char>::new();
    // The assembly is finished when the last worker is done.
    let mut time_taken = 0;

    while !(queue.is_empty() && workers.is_empty()) {
        // Fill up as many workers as possible from items off the queue
        while workers.len() < flags.nworkers && !queue.is_empty() {
            let RChar(step) = queue.pop().unwrap();
            let deadline = time_taken + (step as u8 - b'A' + 1) as u32 + flags.time_offset;
            workers.push((RUint32(deadline), RChar(step)))
        }

        // Step forward, simulating the next worker that would complete their instruction
        let (RUint32(deadline), RChar(instr)) = workers.pop().unwrap();
        time_taken = deadline;
        out.push(instr);

        // Check all dependencies of the completed instruction,
        // and queue up any that have had all their pre-instructions completed
        // (signalled by having zero in-degrees).
        if let Some(mut children) = edges.remove(&instr) {
            // Where `children` is a lexographically ascending-sorted BinaryHeap.
            while let Some(child) = children.pop() {
                let mut d = indegrees.get_mut(&child.0).unwrap();
                *d -= 1;
                if *d == 0 {
                    queue.push(child);
                }
            }
        }
        // eprintln!("{:?}", &queue);
    }
    let order_of_steps = out.iter().collect::<String>();
    Output{ order_of_steps, time_taken }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn example_1() {
        let expect = "CABDFE"; // Order steps should complete in
        assert_eq!(part1(&TEST), expect);
    }

    #[test]
    fn example_2() {
        let flags = Flags{ nworkers: 2, time_offset: 0 };
        let expect = 15; // With 2 workers and a dilated time sampling, it takes 15 seconds
        assert_eq!(part2(&TEST, flags), expect);
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    let flags = Flags{ nworkers: 5, time_offset: 60 };
    println!("part2: {}", part2(&input, flags));
}
