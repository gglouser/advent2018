use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn parse_input(s: &str) -> Vec<(u8, u8)> {
    s.lines().map(|line| {
        let words = line.split_whitespace().collect::<Vec<_>>();
        (words[1].as_bytes()[0], words[7].as_bytes()[0])
    }).collect()
}

fn execute<F>(task_spec: &[(u8,u8)], workers: u32, step_time_f: F) -> (String, usize)
    where F: Fn(u8) -> usize
{
    let mut prereqs = HashMap::new();
    let mut side_a = HashSet::new();
    let mut side_b = HashSet::new();
    for &(pre, dep) in task_spec.iter() {
        (*prereqs.entry(dep).or_insert(vec![])).push(pre);
        side_a.insert(pre);
        side_b.insert(dep);
    }

    let mut ready: BinaryHeap<_> = side_a.difference(&side_b).map(|&s| Reverse(s)).collect();
    let mut not_ready: HashSet<u8> = side_a.union(&side_b).cloned().collect();
    for Reverse(s) in ready.iter() {
        not_ready.remove(s);
    }

    let mut result = String::new();
    let mut done = HashSet::new();
    let mut in_progress = BinaryHeap::new();
    let mut workers_avail = workers;
    let mut t = 0;

    while !(ready.is_empty() && in_progress.is_empty()) {
        // Start jobs
        while workers_avail > 0 {
            if let Some(Reverse(step)) = ready.pop() {
                in_progress.push(Reverse((t + step_time_f(step), step)));
                workers_avail -= 1;
            } else {
                break;
            }
        }

        // Advance time; finish next job
        if let Some(Reverse((next_t,completed))) = in_progress.pop() {
            t = next_t;
            result.push(char::from(completed));
            done.insert(completed);
            workers_avail += 1;
        }

        // Ready new jobs
        let to_ready: Vec<u8> = not_ready.iter()
            .filter(|&s| prereqs[&s].iter().all(|p| done.contains(&p)))
            .cloned().collect();
        for s in to_ready {
            not_ready.remove(&s);
            ready.push(Reverse(s));
        }
    }

    (result, t)
}

fn solve(input: &str) -> (String, usize) {
    let instrs = parse_input(input);
    let (part1, _) = execute(&instrs, 1, |_| 0);
    let (_, part2) = execute(&instrs, 5, |s| (s - b'A') as usize + 61);
    (part1, part2)
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
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), vec![
                (b'C', b'A'),
                (b'C', b'F'),
                (b'A', b'B'),
                (b'A', b'D'),
                (b'B', b'E'),
                (b'D', b'E'),
                (b'F', b'E'),
            ]);
    }

    #[test]
    fn example() {
        let instrs = parse_input(EXAMPLE);
        let (part1, _) = execute(&instrs, 1, |_| 0);
        assert_eq!("CABDFE", part1);
        let (_, part2) = execute(&instrs, 2, |s| (s - b'A') as usize + 1);
        assert_eq!(15, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day07.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day07.txt"),
                   format!("{:?}", x));
    }
}
