use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
struct Point { x: i16, y: i16 }

impl Point {
    fn new(x: i16, y:i16) -> Point {
        Point{x,y}
    }
    fn manhattan(self, other: Point) -> i16 {
        (self.x - other.x).abs() +
        (self.y - other.y).abs()
    }
}

fn parse(puzzle: &str) -> Vec<Point> {
    puzzle.lines()
        .map(|line| line.split(", ")
             .filter_map(|s| s.parse::<i16>().ok())
             .collect::<Vec<i16>>()
         )
        .map(|xy| Point{ x: xy[0], y: xy[1] })
        .collect::<Vec<_>>()
}

fn find_bounds(coords: &Vec<Point>) -> (Point, Point) {
    (Point::new(0,0), // Assume 0,0 for left,top boundary
     Point::new(coords.iter().max_by_key(|c| c.x).unwrap().x+1,  // right edge
                coords.iter().max_by_key(|c| c.y).unwrap().y+1)) // bottom edge
}

type Grid = Vec<Vec<Option<u8>>>;

fn new_grid(maxs: Point) -> Grid {
    vec![vec![None; maxs.x as usize]; maxs.y as usize]
}

fn debug_show_grid(grid: &Grid, enabled: bool) {
    if !enabled { return }

    let mut char_grid = grid.iter()
        .map(|line| line.iter().map(|c| match *c {
                 Some(c) => b'A' + c,
                 None => b'.',
             })
             .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let s = char_grid.as_mut_slice().join(&b'\n');

    eprintln!("\n{}", String::from_utf8_lossy(&s));
}

// "What is the size of the largest area that isn't infinite?"
fn part1(puzzle: &str, flags: Flags) -> u32 {
    let coords = parse(&puzzle);

    // Find four boundaries
    let (mins, maxs) = find_bounds(&coords);

    // Create a grid with the above boundaries.
    let mut grid = new_grid(maxs);
    // Track region sizes and if a region is infinite.
    let mut counts = vec![0u32; coords.len()];
    let mut infinites = HashSet::new();

    // Loop over every location in the grid
    for y in mins.y..maxs.y {
        for x in mins.x..maxs.x {
            let loc = Point{x, y};

            let (mut neighbor, mut min_dist) = (None, i16::max_value());

            // Find the closest coordinate - ties mean no one wins
            for (idx, point) in coords.iter().enumerate() {
                let dist = loc.manhattan(*point);
                if dist < min_dist {
                    neighbor = Some(idx as u8);
                    min_dist = dist;
                } else if dist == min_dist {
                    neighbor = None;
                };
            }
            if let Some(neighbor) = neighbor {
                // check if this location makes a coordinate 'infinite'
                if loc.x == mins.x || loc.x == maxs.x-1 ||
                   loc.y == mins.y || loc.y == maxs.y-1
                {
                    infinites.insert(neighbor);
                }

                counts[neighbor as usize] += 1;
            }
            grid[y as usize][x as usize] = neighbor;
        }
    }
    debug_show_grid(&grid, flags.show_grid);

    counts.into_iter()
        .enumerate()
        .filter_map(|(id,size)| if infinites.contains(&(id as u8)) { None } else { Some(size) })
        .max().unwrap()
}

// "What is the size of the region containing all locations which have a total distance
//  to all given coordinates of less than 10000?"
fn part2(puzzle: &str, flags: Flags, threshold: i32) -> u32 {
    let coords = parse(&puzzle);
    let (mins, maxs) = find_bounds(&coords);
    let mut grid = new_grid(maxs);
    for y in mins.y..maxs.y {
        for x in mins.x..maxs.x {
            let loc = Point{x, y};
            let score = coords.iter().map(|coord| coord.manhattan(loc) as i32).sum::<i32>();
            if score < threshold {
                grid[y as usize][x as usize] = Some(0);
            }
        }
    }
    debug_show_grid(&grid, flags.show_grid);

    // The 'region' here is guaranteed to one contiguous blob,
    // so counting is straight forward.
    // If there were multiple 'safe' regions and the puzzle was to find the largest one,
    // then a bfs (flood-filling) approach could be substituted in here.
    grid.iter().flatten().filter(|o| o.is_some()).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    static OPTS: Flags = Flags{ show_grid: true };
    static TEST: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn example_1() {
        let expect = 17; // 'E' (fifth line) has a finite area of 17
        assert_eq!(part1(&TEST, OPTS), expect);
    }

    #[test]
    fn example_2() {
        let threshold = 32;
        let expect = 16; // Region within the threshold has an area of 16
        assert_eq!(part2(&TEST, OPTS, threshold), expect);
    }
}


// Lazy initialized singletons would require a package
// However, avoiding singletons in general isn't too bad.
//
// This seems ok for passing around feature flags - there's only one
// flag now, but the idea is sound enough.
#[derive(Copy,Clone)]
struct Flags {
    show_grid: bool,
}

fn parse_args() -> Flags {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let show_grid = args.iter().any(|s| s == "--show-grid");
    Flags{show_grid}
}

fn main() {
    // Save the diagrams: `cargo run --release -- --show-grid 2> /tmp/maps`
    let flags = parse_args();

    let input = include_str!("../input");
    println!("part1: {}", part1(&input, flags));
    let threshold = 10_000;
    println!("part2: {}", part2(&input, flags, threshold));
}
