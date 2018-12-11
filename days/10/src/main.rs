type Pair = (i32,i32);

static STAR_ON:  u8 = b'#';
static STAR_OFF: u8 = b' ';

struct Star {
    pos: Pair, // x, y
    vel: Pair, // velocity of x, & y
}

// Star patterns will not be displayed if they are larger than reasonable.
// I took a guess at how large that should be, and 50 works out well.
static VIEWPORT_HEIGHT_THRESHOLD: i32 = 50;

fn record_the_stars(stars: &[Star], time: usize) {
    let all_pos = stars.iter().map(|sp| sp.pos);

    // Calculate bbox for the star pattern, abort if the pattern is too large to display.
    let (minx, miny, xdim, ydim) = {
        let first = stars[0].pos;
        let (mut minx,mut maxx, mut miny,mut maxy) = (first.0, first.0, first.1, first.1);
        for star in all_pos.clone().skip(1) {
            minx = i32::min(minx, star.0);
            maxx = i32::max(maxx, star.0);
            miny = i32::min(miny, star.1);
            maxy = i32::max(maxy, star.1);
        }

        if maxy - miny > VIEWPORT_HEIGHT_THRESHOLD { return }

        (minx, miny, (maxx - minx + 1) as usize, (maxy - miny + 1) as usize )
    };

    // Create one contiguous memory block for the matrix display,
    // using offsets to index into the grid.
    let mut grid = vec![STAR_OFF; xdim*ydim];

    // Translate points
    for (x,y) in all_pos {
        let xp = (x - minx) as usize;
        let yp = (y - miny) as usize;
        grid[xp + (yp * xdim)] = STAR_ON;
    }

    // Convert the byte display into a rectangle-String
    let grid = grid.chunks(xdim)
        .map(|bytes| String::from_utf8_lossy(bytes))
        .collect::<Vec<_>>();
    let grid = grid.join("\n");
    println!("{}\nTime: {}\n", grid, time);
}

// Run the star simulation for one step
fn next(stars: &mut [Star]) {
    for Star { pos, vel } in stars {
        *pos = (pos.0 + vel.0, pos.1 + vel.1)
    }
}

struct Flags { iters: usize, recording_starts_at: usize }

// Part1: "What message will eventually appear in the sky?"
// Part2: "exactly how many seconds would they have needed to wait
//         for that message to appear?"
fn part1(puzzle: &str, flags: Flags) -> () {
    let cbodies = &mut puzzle.lines()
        .map(|line| {
            // Parse pos/velocity pairs (see tests for example input)
            let nums = line
                .split(|c| c == '<' || c == ',' || c == '>')
                .filter_map(|snum| snum.trim().parse().ok())
                .collect::<Vec<i32>>();
            Star{ pos: (nums[0],nums[1]), vel: (nums[2],nums[3]) }
        }).collect::<Vec<_>>();

    // Run the simulation to completion.
    for time in 0..flags.iters {
        if time > flags.recording_starts_at {
            record_the_stars(cbodies.as_slice(), time);
        }
        next(cbodies.as_mut_slice());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let test = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        // Verify manually - should read "HI".
        part1(&test, Flags{ iters: 4, recording_starts_at: 2 });
    }
}

fn main() {
    let input = include_str!("../input");
    // There's no way other than to guess at how long to run this.
    //
    // At first, I started with 100k runs, but by printing out the bbox each step,
    // I could see they "converge" around 10,650~, with the input provided.
    part1(&input, Flags{ iters: 11_000, recording_starts_at: 10_500 });
    // part1(&input, Flags{ iters: 100_000, recording_starts_at: 0 });
}
