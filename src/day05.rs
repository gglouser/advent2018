fn parse_input(s: &str) -> &str {
    s.trim()
}

fn test_react(a: u8, b: u8) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn collapsed_len_i<I: Iterator<Item=u8>>(polymer: I) -> usize {
    let mut units = vec![];
    for u in polymer {
        match units.last() {
            Some(&last_u) if test_react(last_u, u) => { units.pop(); },
            _ => units.push(u),
        }
    }
    units.len()
}

fn collapsed_len(polymer: &str, ignore: Option<u8>) -> usize {
    if let Some(ig) = ignore {
        collapsed_len_i(polymer.bytes().filter(|u| u.to_ascii_lowercase() != ig))
    } else {
        collapsed_len_i(polymer.bytes())
    }
}

fn solve(input: &str) -> (usize, usize) {
    let polymer = parse_input(input);
    let part1 = collapsed_len(polymer, None);
    let shortest = (b'a'..=b'z').map(|ignore| collapsed_len(polymer, Some(ignore))).min().unwrap();
    (part1, shortest)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "dabAcCaCBAcCcaDA\n";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), "dabAcCaCBAcCcaDA");
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(10, part1);
        assert_eq!(4, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day05.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day05.txt"),
                   format!("{:?}", x));
    }
}
