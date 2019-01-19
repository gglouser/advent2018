use std::collections::HashSet;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_input(s: &str) -> Vec<Claim> {
    let re_claim = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    s.lines().map(|line| {
            let c = re_claim.captures(line).unwrap();
            Claim {
                id: c[1].parse().unwrap(),
                left: c[2].parse().unwrap(),
                top: c[3].parse().unwrap(),
                width: c[4].parse().unwrap(),
                height: c[5].parse().unwrap(),
                }
        }).collect()
}

fn solve(input: &str) -> (usize, usize) {
    let claims = parse_input(input);

    let mut grid = vec![vec![vec![]; 1000]; 1000];
    let mut clean_claims = HashSet::new();
    for claim in claims.iter() {
        let mut is_clean = true;
        for row in grid.iter_mut().skip(claim.top).take(claim.height) {
            for spot in row.iter_mut().skip(claim.left).take(claim.width) {
                if !spot.is_empty() {
                    clean_claims.remove(&spot[0]);
                    is_clean = false;
                }
                spot.push(claim.id);
            }
        }
        if is_clean {
            clean_claims.insert(claim.id);
        }
    }

    let multis = grid.iter().map(|row| row.iter().filter(|&cs| cs.len() > 1).count()).sum();

    assert_eq!(clean_claims.len(), 1);
    let clean = clean_claims.drain().next().unwrap();

    (multis, clean)
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
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), vec![
            Claim{id:1,left:1,top:3,width:4,height:4},
            Claim{id:2,left:3,top:1,width:4,height:4},
            Claim{id:3,left:5,top:5,width:2,height:2},
            ]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(4, part1);
        assert_eq!(3, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day03.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day03.txt"),
                   format!("{:?}", x));
    }
}
