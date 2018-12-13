use std::collections::HashMap;

type Planters = HashMap<i32, bool>;
type Rules = HashMap<u8, bool>;
type Bounds = (i32,i32);

fn _print(plants: &Planters, gen_id: impl std::fmt::Display) {
    let keys = &mut plants.keys().collect::<Vec<_>>();
    keys.sort();

    let s = keys.iter().map(|b| if plants[b] {'#'} else {'.'}).collect::<String>();
    println!("{}: {}", gen_id, s);
}

fn next_gen(planters: &mut Planters, rules: &Rules, bounds: Bounds) -> Bounds {
    let mut new_bounds = bounds;

    // Plant pattern is a 'bitset' of five bits, that gets shifted onto for each
    // pot index. e.g. if the next pot has a plant, 00010 would become 00101.
    let mut plant_pattern = 0;
    for plant_pos in bounds.0 - 2..bounds.1 + 2 {
        let next_bit = *planters.get(&(plant_pos+2)).unwrap_or(&false);
        plant_pattern = plant_pattern << 1 | (next_bit as u8);
        plant_pattern &= 0x1F; // strip to 5 bits

        //println!("plants: {:05b}", plant_pattern);
        let is_plant = *rules.get(&plant_pattern).unwrap_or(&false);
        if plant_pos < bounds.0 || plant_pos > bounds.1 {
            if is_plant {
                new_bounds = (
                    std::cmp::min(new_bounds.0, plant_pos),
                    std::cmp::max(new_bounds.1, plant_pos));
                planters.insert(plant_pos, is_plant);
            }
        } else {
            planters.insert(plant_pos, is_plant);
        }
    }
    new_bounds
}

// "After 20 generations, what is the sum of the numbers of all pots which
//  contain a plant?"
//
// "After fifty billion (50000000000) generations, what is the sum of the
//  numbers of all pots which contain a plant?"
fn simulate(puzzle: &str, generations: u64) -> i64 {
    // Build a map of pot index -> true/false 'has a plant'
    let planters = puzzle.split_whitespace().skip(2).next().unwrap().trim().as_bytes();
    let mut planters: Planters =
        (0..i32::max_value())
        .zip(
            planters.into_iter().map(|&b| b == b'#')
         ).collect();

    // Build a map of rules transforming a 5-bit pattern
    // as a u8 (acting as a bitset) -> true/false 'becomes a plant'
    let rules: Rules = puzzle.lines().skip(2)
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let a = parts[0].chars().fold(0, |n, ch| (n << 1) | ((ch == '#') as u8));
            let b = parts[2] == "#";
            (a, b)
        }).collect();

    // Track the first & last planter box as they grow outward
    let mut bounds: Bounds = (0, planters.len() as i32);
    // Track score each generation, looking for when the delta between
    // two generation's scores matches - at which point, the pot pattern
    // just grows out as a gliders, forever.
    let (mut score, mut delta) = (0i64, 0i64);

    for i in 0..generations {
        bounds = next_gen(&mut planters, &rules, bounds);

        // Calculate score by adding the indexes of pots with a plant in them
        let new_score = planters.iter()
            .fold(0i64, |sum, (&idx,&plant)| if plant { sum + idx as i64 } else { sum });

        // If the delta hasn't changed, then a cycle has been reached.
        //
        // This trick became apparent when printing out the first 100 generations.
        let new_delta = new_score - score;
        if new_delta == delta {
            println!("Growth stabilized after gen<{}>", i-1);
            // The final score just adds the score delta for the remaining 4.99 billion
            // generations to the fixed portion of the score (from one generation prior).
            return new_score + new_delta * (generations - i - 1) as i64;
        }
        delta = new_delta;
        score = new_score;
    }
    score
}

#[test]
fn example_1() {
    let test = include_str!("../test_input");
    let expect = 325;
    assert_eq!(simulate(&test, 20), expect);
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", simulate(&input, 20));
    println!("part2: {}", simulate(&input, 50_000_000_000));
}
