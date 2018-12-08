#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    data: Vec<u32>,
}

// Recursively build a tree structure.
// The input vector is described in the puzzle instructions
// (two fields for header, then variable amount of stuff).
fn read_node(v: &mut Vec<u32>) -> Node {
    let header = v.drain(0..2).collect::<Vec<_>>();
    let (nchild, ndata) = (header[0], header[1]);

    let mut children = Vec::new();
    for _ in 0..nchild {
        children.push(read_node(v));
    }
    Node {
        children,
        data: v.drain(0..ndata as usize).collect::<Vec<_>>(),
    }
}

fn parse(puzzle: &str) -> Node {
    let v = &mut puzzle.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();
    read_node(v)
}

// "What is the sum of all metadata entries?"
fn part1(puzzle: &str) -> u32 {
    let tree = parse(puzzle);

    fn visit(node: &Node) -> u32 {
        let mut sum: u32 = node.data.iter().sum();
        for child in &node.children {
            sum += visit(child);
        }
        sum
    }
    visit(&tree)
}

// "What is the value of the root node?"
fn part2(puzzle: &str) -> u32 {
    let tree = parse(puzzle);

    // Only sum nodes without children, or the indexed-via-metadata children
    fn visit(node: &Node) -> u32 {
        if node.children.is_empty() {
            node.data.iter().sum()
        } else {
            let mut sum: u32 = 0;
            for data in &node.data {
                if let Some(child) = node.children.get((*data-1) as usize) {
                    sum += visit(child);
                }
            }
            sum
        }
    }
    visit(&tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn example_1() {
        let expect = 138; // (Check)sum of all metadata
        assert_eq!(part1(&TEST), expect);
    }

    #[test]
    fn example_2() {
        let expect = 66; // (Check)sum of referenced children's metadata
        assert_eq!(part2(&TEST), expect);
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
