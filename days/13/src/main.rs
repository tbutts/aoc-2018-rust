use std::collections::HashSet;

// parse the puzzle input tiles into a 2d matrix of track tiles + list of mine carts
fn parse(puzzle: &str) -> (Vec<Cart>, Vec<Vec<Track>>) {
    let mut carts = Vec::<Cart>::new();
    let map = puzzle.lines().enumerate().map(|(y, line)| {
        line.as_bytes().iter().enumerate().map(|(x, byte)| {
            match byte {
                b'-' | b'|' => Track::Rail,
                b' '        => Track::Empty,
                b'\\'       => Track::CornerBkwd,
                b'/'        => Track::CornerFwd,
                b'+'        => Track::Intersection,
                _           => {
                    // Assume any carts have a rail line under them
                    carts.push(Cart::new(
                        Loc{x,y},
                        match byte {
                            b'^' => Dir::North,
                            b'>' => Dir::East,
                            b'v' => Dir::South,
                            b'<' => Dir::West,
                            _    => panic!("unexpected tile: ({},{}) = {}",
                                            x, y, char::from(*byte)),
                        }));

                    Track::Rail
                },
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let first_line_len = map[0].len();
    assert!(map.iter().skip(1).all(|line| line.len() == first_line_len));
    (carts, map)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
enum Dir { North = 0, East, South, West, }
const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

#[derive(Copy, Clone)]
enum Turn { Left = 0, Straight, Right, }
const TURNS: [Turn; 3] = [Turn::Left, Turn::Straight, Turn::Right];

// navigate moves the cart in it's face direction. `move` is a keyword in Rust, hence `navigate`.
fn navigate(loc: &Loc, dir: Dir) -> Loc {
    match dir {
        Dir::North => Loc{x: loc.x,     y: loc.y - 1},
        Dir::East  => Loc{x: loc.x + 1, y: loc.y},
        Dir::South => Loc{x: loc.x,     y: loc.y + 1},
        Dir::West  => Loc{x: loc.x - 1, y: loc.y},
    }
}

// pivot rotates the face direction (either counter-clockwise, clockwise, or no change)
fn pivot(dir: Dir, turn: Turn) -> Dir {
    match turn {
        Turn::Left  => DIRS[(dir as usize + DIRS.len() - 1) % DIRS.len()],
        Turn::Right => DIRS[(dir as usize + 1) % DIRS.len()],
        Turn::Straight => dir,
    }
}

// The Track / map is made up of these set pieces.
enum Track { Rail, CornerBkwd, CornerFwd, Intersection, Empty, }

// Mine Carts have a position, direction they're facing, and record of the next turn they'll make.
struct Cart<'a> {
    loc: Loc,
    dir: Dir,
    turns: Box<Iterator<Item=&'a Turn>>,
}

impl<'a> Cart<'a> {
    fn new(loc: Loc, dir: Dir) -> Cart<'a> {
        Cart {loc, dir, turns: Box::new(TURNS.iter().cycle()), }
    }

    fn tick(&mut self, map: &[Vec<Track>]) {
        // Move foward, then turn if on a '/\' corner or '+' intersection
        self.loc = navigate(&self.loc, self.dir);
        let env = &map[self.loc.y][self.loc.x];
        match env {
            Track::Rail => {},
            Track::CornerFwd => { // forward slash '/'
                self.dir = match self.dir {
                    Dir::East  | Dir::West  => pivot(self.dir, Turn::Left),
                    Dir::North | Dir::South => pivot(self.dir, Turn::Right),
                }
            },
            Track::CornerBkwd => { // backward slash '\'
                self.dir = match self.dir {
                    Dir::East  | Dir::West  => pivot(self.dir, Turn::Right),
                    Dir::North | Dir::South => pivot(self.dir, Turn::Left),
                }
            },
            Track::Intersection => {
                // Take the next 'memorized' series of turns
                self.dir = pivot(self.dir, *self.turns.next().expect("Next turn"));
            },
            Track::Empty => panic!("Going off the rails!"),
        }
    }
}

const MAX_TICKS: usize = 200_000;

// "...you'd like to know the location of the first crash."
fn part1(puzzle: &str) -> Loc {
    let (mut carts, map) = parse(puzzle);

    let mut cart_positions: HashSet<Loc> = carts.iter().map(|cart| cart.loc).collect();

    for _tick in 0..MAX_TICKS {
        carts.sort_by_key(|c| (c.loc.y, c.loc.x));

        for mut cart in carts.iter_mut() {
            cart_positions.remove(&cart.loc);

            cart.tick(map.as_slice());

            let spot_taken = !cart_positions.insert(cart.loc);
            if spot_taken {
                println!("Crash detected! @ {:?}", cart.loc);
                return cart.loc;
            }
        }
    }
    panic!("The mine carts weren't supposed to be safe!");
}

// "What is the location of the last cart at the end of the first tick where
//  it is the only cart left?"
fn part2(puzzle: &str) -> Loc {
    let (mut carts, map) = parse(puzzle);

    let mut cart_positions: HashSet<Loc> = carts.iter().map(|cart| cart.loc).collect();
    let mut collisions = HashSet::new();

    for _tick in 0..MAX_TICKS {
        carts.sort_by_key(|c| (c.loc.y, c.loc.x));

        for mut cart in carts.iter_mut() {
            cart_positions.remove(&cart.loc);
            if collisions.contains(&cart.loc) { continue; }

            cart.tick(map.as_slice());

            let spot_taken = !cart_positions.insert(cart.loc);
            if spot_taken {
                println!("Crash detected! @ {:?}", cart.loc);
                collisions.insert(cart.loc);
                cart_positions.remove(&cart.loc);
            }
        }

        // Prune crashed carts
        if collisions.len() > 0 {
            carts = carts.into_iter().filter(|cart| !collisions.contains(&cart.loc)).collect();
            if carts.len() == 1 {
                return carts[0].loc;
            }
            collisions.clear();
        }
    }
    panic!("The mine carts weren't supposed to be safe!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let test = include_str!("../test_input");
        let expect = Loc{x:7,y:3};
        assert_eq!(part1(&test), expect);
    }

    #[test]
    fn example_2() {
        let test = include_str!("../test_input2");
        let expect = Loc{x:6,y:4};
        assert_eq!(part2(&test), expect);
    }
}

fn main() {
    let input = include_str!("../input");
    let crash = part1(&input);
    println!("part1: {},{}", crash.x, crash.y);
    let crash = part2(&input);
    println!("part2: {},{}", crash.x, crash.y);
}
