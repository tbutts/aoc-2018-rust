type Serial = u32;

fn calc_power(x: usize, y: usize, grid_serial: Serial) -> i32 {
    let rack_id = x as u32 + 10;
    let mut score: u32 = rack_id * y as u32;
    score += grid_serial;
    score *= rack_id;
    score /= 100;
    score %= 10;
    (score as i32) - 5
}

type Grid = [[i32;300]; 300];

#[derive(Debug)]
struct BestID {
    x: usize,
    y: usize,
    size: usize,
    power: i32,
}

fn find_largest_power_by_dim(grid: &Grid, size: usize) -> BestID {
    let mut largest = BestID{x:0,y:0,size:0,power:0};

    // Naive solution, heavily nested loops that sum all elements of the 3x3 window
    for y in 0..grid.len()-size {
        for x in 0..grid.len()-size {

            let mut power: i32 = 0;
            for y in y..y+size {
                for x in x..x+size {
                    power += grid[y][x];
                }
            }

            if power > largest.power {
                largest = BestID{x:x+1,y:y+1,size,power};
            }

        }
    }
    largest
}

// "What is the X,Y coordinate of the top-left fuel cell of the 3x3 square
//  with the largest total power?"
fn part1(grid_serial: Serial) -> (usize,usize) {
    let mut grid = [[0i32; 300]; 300];

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            grid[y][x] = calc_power(x+1, y+1, grid_serial);
        }
    }

    let tres = find_largest_power_by_dim(&grid, 3);
    (tres.x, tres.y)
}

type Grid2 = [[i32;301]; 301];

// moving_sums produces the power cell grid with each cell containing the
// sum of all previous (left & above) power levels combined.
fn moving_sums(grid_serial: Serial) -> Grid2 {
    let mut grid = [[0i32; 301]; 301];
    // Carefully looking at the problem description, I found that the top row
    // and left-most column can be left as zeros, removing the need for manual
    // bounds checking. Coordinates always range from 1 to 300, inclusive.
    for y in 1..grid.len() {
        for x in 1..grid.len() {
            let power = calc_power(x, y, grid_serial);
            grid[y][x] = power
                - grid[y-1][x-1] + grid[y-1][x] + grid[y][x-1];
        }
    }
    grid
}

// "What is the X,Y,size identifier of the square with the largest total power?"
fn part2(grid_serial: Serial) -> (usize,usize,usize) {
    let grid = moving_sums(grid_serial);

    let mut largest = BestID{x:0, y:0, size:0, power: i32::min_value()};

    // There's no reason to try beyond a certain size, as the best totals
    // plateau in the 10s or 20s.
    for size in 4..30 {
        for y in 1..grid.len()-size {
            for x in 1..grid.len()-size {
                // Only select the value information bound by the window size
                // being examined.
                let power = grid[y][x]
                    + grid[y+size][x+size] - grid[y+size][x] - grid[y][x+size];
                if power > largest.power {
                    largest = BestID{x,y,size,power};
                }
            }
        }
    }
    (largest.x+1, largest.y+1, largest.size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_calc_power() {
        struct Test { x: usize, y: usize, grid_serial: Serial }
        let tests = vec!(
            (Test{ x:  3, y:  5, grid_serial:  8 },  4),
            (Test{ x:122, y: 79, grid_serial: 57 }, -5),
            (Test{ x:217, y:196, grid_serial: 39 },  0),
            (Test{ x:101, y:153, grid_serial: 71 },  4),
        );
        for (test, expect) in tests {
            assert_eq!(calc_power(test.x, test.y, test.grid_serial), expect);
        }
    }

    #[test]
    fn example_1() {
        let tests = vec!(
            (18, (33,45)),
            (42, (21,61)),
        );
        for (grid_serial, expect) in tests {
            assert_eq!(part1(grid_serial), expect);
        }
    }

    #[test]
    fn example_2() {
        let tests = vec!(
            (18,  (90,269,16)),
            (42, (232,251,12)),
        );
        for (grid_serial, expect) in tests {
            assert_eq!(part2(grid_serial), expect);
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let grid_serial: Serial = input.trim().parse().unwrap();
    println!("part1: {:?}", part1(grid_serial));
    println!("part2: {:?}", part2(grid_serial));
}
