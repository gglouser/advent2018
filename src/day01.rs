use std::collections::HashSet;

fn parse_input(s: &str) -> Vec<i32> {
    s.lines().map(|m| m.parse().unwrap()).collect()
}

fn first_repeat(freq_changes: &[i32]) -> i32 {
    freq_changes.iter().cycle()
        .scan((0, HashSet::new()), |(current,seen), &change| {
            if seen.insert(*current) {
                *current += change;
                Some(*current)
            } else {
                None
            }
        })
        .last().unwrap()
}

fn solve(input: &str) -> (i32, i32) {
    let freq_changes = parse_input(input);
    let freq = freq_changes.iter().sum();
    let repeat = first_repeat(&freq_changes);
    (freq, repeat)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("+1\n-2\n+3\n+1\n"),
                   vec![1, -2, 3, 1]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve("+1\n-2\n+3\n+1\n");
        assert_eq!(3, part1);
        assert_eq!(2, part2);
    }

    #[test]
    fn example2() {
        assert_eq!(0, first_repeat(&[1, -1]));
        assert_eq!(10, first_repeat(&[3, 3, 4, -2, -4]));
        assert_eq!(5, first_repeat(&[-6, 3, 8, 5, -6]));
        assert_eq!(14, first_repeat(&[7, 7, -2, -7, -4]));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day01.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day01.txt"),
                   format!("{:?}", x));
    }
}
