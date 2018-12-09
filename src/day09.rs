use std::collections::VecDeque;

fn parse_input(s: &str) -> (usize, u32) {
    let words = s.split_whitespace().collect::<Vec<_>>();
    (words[0].parse().unwrap(), words[6].parse().unwrap())
}

// *Current marble* is in the back.
struct MarbleCircle(VecDeque<u32>);

impl MarbleCircle {
    fn new() -> Self {
        let mut deq = VecDeque::new();
        deq.push_back(0);
        MarbleCircle(deq)
    }

    fn place(&mut self, m: u32) {
        self.0.push_back(m);
    }

    fn take(&mut self) -> u32 {
        self.0.pop_back().unwrap()
    }

    fn cw(&mut self) {
        let m = self.0.pop_front().unwrap();
        self.0.push_back(m);
    }

    fn ccw(&mut self) {
        let m = self.0.pop_back().unwrap();
        self.0.push_front(m);
    }
}

fn marble_game(num_players: usize, last_marble: u32) -> Vec<u32> {
    let mut scores = vec![0; num_players];
    let mut cur_player = 0;
    let mut circle = MarbleCircle::new();
    for n in 1..=last_marble {
        if n % 23 != 0 {
            circle.cw();
            circle.place(n);
        } else {
            scores[cur_player] += n;
            for _ in 0..7 {
                circle.ccw();
            }
            scores[cur_player] += circle.take();
            circle.cw();
        }
        cur_player = (cur_player + 1) % num_players;
    }
    scores
}

fn winning_score(n: usize, m: u32) -> u32 {
    let scores = marble_game(n, m);
    *scores.iter().max().unwrap()
}

fn solve(input: &str) -> (u32, u32) {
    let (num_players, last_marble) = parse_input(input);
    let winner1 = winning_score(num_players, last_marble);
    let winner2 = winning_score(num_players, 100*last_marble);
    (winner1, winner2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE : &'static str = "5 players; last marble is worth 25 points\n";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE), (5, 25));
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(32, part1);
        assert_eq!(37923, part2);
    }

    #[test]
    fn example2() {
        assert_eq!(8317, winning_score(10, 1618));
        assert_eq!(146373, winning_score(13, 7999));
        assert_eq!(2764, winning_score(17, 1104));
        assert_eq!(54718, winning_score(21, 6111));
        assert_eq!(37305, winning_score(30, 5807));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day09.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day09.txt"),
                   format!("{:?}", x));
    }
}
