use std::collections::HashMap;
use regex::Regex;

fn parse_input(s: &str) -> Vec<(&str, &str)> {
    s.lines().map(|line| (&line[1..17], &line[19..])).collect()
}

fn process_events(events: &[(&str, &str)]) -> HashMap<u32, Vec<(u32,u32)>> {
    let re_minute = Regex::new(r"....-..-.. ..:(\d\d)").unwrap();
    let re_guard = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut guards = HashMap::new();
    let mut cur_guard = 0;
    let mut sleep_time = 0;
    for (time, event) in events {
        if let Some(c) = re_guard.captures(event) {
            cur_guard = c[1].parse().unwrap();
        } else if *event == "falls asleep" {
            let c = re_minute.captures(time).unwrap();
            sleep_time = c[1].parse().unwrap();
        } else if *event == "wakes up" {
            let c = re_minute.captures(time).unwrap();
            let wake_time = c[1].parse().unwrap();
            let guard = guards.entry(cur_guard).or_insert_with(|| vec![]);
            guard.push((sleep_time, wake_time));
        } else {
            panic!("unknown event: {}", event);
        }
    }

    guards
}

fn track_sleep(sleeps: &[(u32,u32)]) -> (u32, u32, u32) {
    let mut total = 0;
    let mut minutes = [0; 60];
    for &(s,w) in sleeps.iter() {
        total += w - s;
        for m in s..w {
            minutes[m as usize] += 1;
        }
    }
    let most_minute = (0..60).max_by_key(|&i| minutes[i]).unwrap();
    (total, most_minute as u32, minutes[most_minute])
}

fn solve(input: &str) -> (u32, u32) {
    let mut events = parse_input(input);
    events.sort();

    let guards = process_events(&events);
    let sleep_sched = guards.iter().map(|(gid,sleeps)| {
            let (total, most_minute, most_min_count) = track_sleep(&sleeps);
            (gid, total, most_minute, most_min_count)
        }).collect::<Vec<_>>();

    // Part 1
    let most_sleep = sleep_sched.iter().max_by_key(|x| x.1).unwrap();
    let strat1 = most_sleep.0 * most_sleep.2;

    // Part 2
    let freq_sleep = sleep_sched.iter().max_by_key(|x| x.3).unwrap();
    let strat2 = freq_sleep.0 * freq_sleep.2;

    (strat1, strat2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "\
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
[1518-11-05 00:55] wakes up
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
            vec![("1518-11-01 00:00", "Guard #10 begins shift"),
                 ("1518-11-01 00:05", "falls asleep"),
                 ("1518-11-01 00:25", "wakes up"),
                 ("1518-11-01 00:30", "falls asleep"),
                 ("1518-11-01 00:55", "wakes up"),
                 ("1518-11-01 23:58", "Guard #99 begins shift"),
                 ("1518-11-02 00:40", "falls asleep"),
                 ("1518-11-02 00:50", "wakes up"),
                 ("1518-11-03 00:05", "Guard #10 begins shift"),
                 ("1518-11-03 00:24", "falls asleep"),
                 ("1518-11-03 00:29", "wakes up"),
                 ("1518-11-04 00:02", "Guard #99 begins shift"),
                 ("1518-11-04 00:36", "falls asleep"),
                 ("1518-11-04 00:46", "wakes up"),
                 ("1518-11-05 00:03", "Guard #99 begins shift"),
                 ("1518-11-05 00:45", "falls asleep"),
                 ("1518-11-05 00:55", "wakes up"),
            ]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(240, part1);
        assert_eq!(4455, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day04.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day04.txt"),
                   format!("{:?}", x));
    }
}
