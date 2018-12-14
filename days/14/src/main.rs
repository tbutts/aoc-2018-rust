// "What are the scores of the ten recipes immediately after the number of
//  recipes in your puzzle input?"
fn part1(puzzle: &str) -> String {
    let end: usize = puzzle.trim().parse().unwrap();

    let mut recipes = Vec::<u8>::with_capacity(end + 11);
    recipes.extend([3, 7].iter());

    // Track recipe indexes of the two elves
    let mut elves: [usize; 2] = [0, 1];

    loop {
        let new_recipe: u8 = elves.iter().map(|&elf_pos| recipes[elf_pos]).sum();
        // Add the digits from the 'recipe' sum to the list, highest digit first
        if new_recipe >= 10 {
            recipes.push(new_recipe / 10);
        }
        recipes.push(new_recipe % 10);

        // Move elves forward by their position + recipe score + 1
        for elf in elves.iter_mut() {
            *elf = (*elf + (recipes[*elf] + 1) as usize) % recipes.len();
        }

        if recipes.len() > end + 10 {
            return recipes[end..end+10].iter().map(|b| b.to_string()).collect::<String>()
        }
    }
}


// "How many recipes appear on the scoreboard to the left of the score sequence
//  in your puzzle input?"
fn part2(puzzle: &str) -> usize {
    let score_seq: Vec<u8> = puzzle.trim()
        .chars()
        .flat_map(|c| c.to_string().parse().ok())
        .collect();

    let mut recipes = Vec::<u8>::new();
    recipes.extend([3, 7].iter());

    let mut elves: [usize; 2] = [0, 1];

    loop {
        let new_recipe: u8 = elves.iter().map(|&elf_pos| recipes[elf_pos]).sum();
        if new_recipe >= 10 {
            recipes.push(new_recipe / 10);
        }
        recipes.push(new_recipe % 10);

        for elf in elves.iter_mut() {
            *elf = (*elf + (recipes[*elf] + 1) as usize) % recipes.len();
        }

        if recipes.len() < score_seq.len()+2 { continue; }

        let end = recipes.len()-score_seq.len();
        // End when the input sequence is generated in the list of recipes
        //
        // I read the problem over and over, and I'm still not sure this is
        // the most simple way to perform this check, or if the sequence from
        // part1 could just be reused.
        for i in end-2..end {
            let seq = &recipes[i..i+score_seq.len()];
            if seq == score_seq.as_slice() {
                return i
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let tests = vec![
            ("9",    "5158916779"),
            ("5",    "0124515891"),
            ("18",   "9251071085"),
            ("2018", "5941429882"),
        ];
        for (test, expect) in tests {
            assert_eq!(part1(&test), expect);
        }
    }

    #[test]
    fn example_2() {
        let tests = vec![
            ("51589", 9),
            ("01245", 5),
            ("92510", 18),
            ("59414", 2018),
        ];
        for (test, expect) in tests {
            assert_eq!(part2(&test), expect);
        }
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    // +20million iterations in 600ms on an old laptop!
}
