use std::collections::HashMap;

fn parse_input(s: &str) -> Vec<&str> {
    s.lines().collect()
}

fn char_counts(s: &str) -> HashMap<u8,usize> {
    let mut counts = HashMap::new();
    for c in s.bytes() {
        let e = counts.entry(c).or_insert(0);
        *e += 1;
    }
    counts
}

fn checksum(box_ids: &[&str]) -> usize {
    let counts = box_ids.iter().map(|id| char_counts(id)).collect::<Vec<_>>();
    let twos = counts.iter().filter(|c| c.values().any(|&v| v == 2)).count();
    let threes = counts.iter().filter(|c| c.values().any(|&v| v == 3)).count();
    twos*threes
}

fn common_chars(s1: &str, s2: &str) -> String {
    s1.chars().zip(s2.chars())
        .filter_map(|(c,d)| if c == d { Some(c) } else { None })
        .collect()
}

fn find_boxes(box_ids: &[&str]) -> String {
    for (i, id1) in box_ids.iter().enumerate() {
        for id2 in box_ids[i+1..].iter() {
            let common = common_chars(id1, id2);
            if common.len() == id1.len() - 1 {
                return common;
            }
        }
    }
    unreachable!()
}

fn solve(input: &str) -> (usize, String) {
    let box_ids = parse_input(input);
    (checksum(&box_ids), find_boxes(&box_ids))
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
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

    const EXAMPLE2 : &'static str = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
                   vec!["abcdef","bababc","abbcde","abcccd","aabcdd","abcdee","ababab"]);
    }

    #[test]
    fn example() {
        assert_eq!(12, checksum(&parse_input(EXAMPLE)));
    }

    #[test]
    fn example2() {
        assert_eq!("fgij", find_boxes(&parse_input(EXAMPLE2)));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day02.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day02.txt"),
                   format!("{:?}", x));
    }
}
