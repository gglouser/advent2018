struct RecipeTracker {
    recipes: Vec<usize>,
    elf1: usize,
    elf2: usize,
}

impl RecipeTracker {
    fn new() -> Self {
        RecipeTracker { recipes: vec![3,7], elf1: 0, elf2: 1 }
    }

    fn step(&mut self) {
        let sum = self.recipes[self.elf1] + self.recipes[self.elf2];
        if sum >= 10 {
            self.recipes.push(1);
        }
        self.recipes.push(sum % 10);
        self.elf1 = self.pick_new_recipe(self.elf1);
        self.elf2 = self.pick_new_recipe(self.elf2);
    }

    fn pick_new_recipe(&self, i: usize) -> usize {
        (i + 1 + self.recipes[i]) % self.recipes.len()
    }

    fn scores_after(&mut self, n: usize) -> String {
        while self.recipes.len() < n + 10 {
            self.step();
        }
        self.recipes[n..n+10].iter().map(|d| d.to_string()).collect()
    }

    fn scores_before(&mut self, target: &str) -> usize {
        let target = target.bytes().map(|c| (c - b'0') as usize).collect::<Vec<_>>();
        for i in 0.. {
            // make sure we have enough recipes
            while self.recipes.len() < i + target.len() {
                self.step();
            }

            // check at i
            if &self.recipes[i..i+target.len()] == target.as_slice() {
                return i;
            }
        }
        unreachable!()
    }
}

fn solve(input: &str) -> (String, usize) {
    let input = input.trim();
    let mut tracker = RecipeTracker::new();
    let part1 = tracker.scores_after(input.parse().unwrap());
    let part2 = tracker.scores_before(input);
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

    #[test]
    fn examples_part1() {
        let mut tracker = RecipeTracker::new();
        assert_eq!("5158916779", tracker.scores_after(9));
        assert_eq!("0124515891", tracker.scores_after(5));
        assert_eq!("9251071085", tracker.scores_after(18));
        assert_eq!("5941429882", tracker.scores_after(2018));
    }

    #[test]
    fn examples_part2() {
        let mut tracker = RecipeTracker::new();
        assert_eq!(9, tracker.scores_before("51589"));
        assert_eq!(5, tracker.scores_before("01245"));
        assert_eq!(18, tracker.scores_before("92510"));
        assert_eq!(2018, tracker.scores_before("59414"));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day14.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day14.txt"),
                   format!("{:?}", x));
    }
}
