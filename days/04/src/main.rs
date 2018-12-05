use std::collections::HashMap;

extern crate chrono;
use chrono::{NaiveDateTime, Timelike};

/* "Oh no... dates..." */

enum Event {
    Begin(u16),
    Sleep,
    Wake,
}

struct Record {
    dt: NaiveDateTime,
    action: Event,
}

fn parse_line(s: &str) -> Record {
    // flimsy parsing for a safe language!
    Record{
        dt: NaiveDateTime::parse_from_str(&s[1..17], "%Y-%m-%d %H:%M")
            .expect("first field->date"),
        action: match &s[19..] {
            "falls asleep" => Event::Sleep,
            "wakes up" => Event::Wake,
            s => Event::Begin(
                // Without using regex, calmly huck the string around until it becomes a number
                s.split_whitespace()
                    .nth(1).unwrap()
                    .trim_start_matches('#')
                    .parse::<u16>().expect("guard id number")),
        }
    }
}

// parse the input puzzle, ensuring chronological ordering
fn parse(puzzle: &str) -> Vec<Record> {
    let mut records = puzzle.lines().map(parse_line).collect::<Vec<_>>();
    records.sort_unstable_by(|a,b| a.dt.cmp(&b.dt));
    records
}

type Schedule = HashMap<u16, [u8; 60]>;

// Builds every guard's schedule.
// Like the table in the description, but with schedules grouped by
// guard id and minute. Counts the overlapping minutes into an array,
// discarding the rest of the date information.
//
// So it looks like: {#99: [0,0,0,1,1,1,2,2,1,0,0,...], #100: [3,2,2,1,1,0,0...], }
fn create_guard_schedule(records: &Vec<Record>) -> Schedule {
    let mut schedule = HashMap::new();

    struct State { guard: u16, sleep_start: usize };
    let init = State{ guard: 0, sleep_start: 0 };

    records.iter().fold(init, |mut state, rec| {
        let now_minute = rec.dt.minute() as usize;

        match rec.action {
            Event::Begin(guard) => state.guard = guard,
            Event::Sleep => { state.sleep_start = now_minute },
            Event::Wake => {
                let timetable = schedule.entry(state.guard).or_insert([0u8; 60]);
                for min in state.sleep_start..now_minute {
                    timetable[min] += 1;
                }
            },
        };
        state
    });
    schedule
}

// Perform the specified strategy to find the ideal guard, and minute to break in!
// Function takes a closure that is used with `max_by_key` over all schedule time-tables.
fn execute(schedule: &Schedule, strategy: impl Fn(&[u8]) -> u32) -> u32 {
    // Identify the guard to target using the strategy provided.
    let (id, sleep_minutes_vec) = schedule.iter()
        .max_by_key(|(_,a)| strategy(&a[..]))
        .unwrap();

    // Now, take that guard's time-table and find the minute to strike.
    let sleepiest_minute = sleep_minutes_vec.iter().enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(&b))
        .unwrap().0;

    *id as u32 * sleepiest_minute as u32
}

// "Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?"
fn part1(puzzle: &str) -> u32 {
    let records = parse(puzzle);
    let schedule = create_guard_schedule(&records);

    fn total_sleep(v: &[u8]) -> u32 { v.iter().map(|n| *n as u32).sum() }

    execute(&schedule, |a| total_sleep(&a[..]))
}

// "Of all guards, which guard is most frequently asleep on the same minute?"
fn part2(puzzle: &str) -> u32 {
    let records = parse(puzzle);
    let schedule = create_guard_schedule(&records);

    fn heaviest_sleep(v: &[u8]) -> u32 { *v.iter().max().unwrap() as u32 };

    execute(&schedule, |a| heaviest_sleep(a))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn example_1() {
        let expect = 10 * 24; // Guard #10 @ minute 24
        assert_eq!(part1(&TEST), expect);
    }

    #[test]
    fn example_2() {
        let expect = 99 * 45; // Guard #99 @ minute 45
        assert_eq!(part2(&TEST), expect);
    }
}

fn main() {
    let input = include_str!("../input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
