fn parse_input(s: &str) -> Vec<u32> {
    s.split_whitespace().map(|word| word.parse().unwrap()).collect()
}

#[derive(Clone, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn new() -> Self {
        Node { children: vec![], metadata: vec![] }
    }

    fn parse<I>(entries: &mut I) -> Self
        where I: Iterator<Item = u32>
    {
        let child_count = entries.next().unwrap();
        let metadata_count = entries.next().unwrap();
        let mut node = Node::new();
        for _ in 0..child_count {
            node.children.push(Node::parse(entries));
        }
        for _ in 0..metadata_count {
            node.metadata.push(entries.next().unwrap());
        }
        node
    }

    fn sum_metadata(&self) -> u32 {
        self.children.iter().map(|c| c.sum_metadata()).sum::<u32>()
            + self.metadata.iter().sum::<u32>()
    }

    fn value(&self) -> u32 {
        if !self.children.is_empty() {
            let subvals = self.children.iter().map(|c| c.value()).collect::<Vec<_>>();
            self.metadata.iter().map(|&i| subvals.get(i as usize - 1).unwrap_or(&0)).sum()
        } else {
            self.metadata.iter().sum()
        }
    }
}

fn solve(input: &str) -> (u32, u32) {
    let input = parse_input(input);
    let nodes = Node::parse(&mut input.iter().cloned());
    (nodes.sum_metadata(), nodes.value())
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
2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
            vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]);
    }

    #[test]
    fn example() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(138, part1);
        assert_eq!(66, part2);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day08.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day08.txt"),
                   format!("{:?}", x));
    }
}
