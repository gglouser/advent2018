use regex::Regex;

type RegType = u32;
type Regs = [RegType; 4];
type Val = usize;

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn from_usize(opcode: usize) -> Self {
        match opcode {
            0  => Opcode::Addr,
            1  => Opcode::Addi,
            2  => Opcode::Mulr,
            3  => Opcode::Muli,
            4  => Opcode::Banr,
            5  => Opcode::Bani,
            6  => Opcode::Borr,
            7  => Opcode::Bori,
            8  => Opcode::Setr,
            9  => Opcode::Seti,
            10 => Opcode::Gtir,
            11 => Opcode::Gtri,
            12 => Opcode::Gtrr,
            13 => Opcode::Eqir,
            14 => Opcode::Eqri,
            15 => Opcode::Eqrr,
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instr(Opcode, Val, Val, Val);

#[derive(Clone, Copy, Debug)]
struct Machine {
    regs: Regs,
}

impl Machine {
    fn new(regs: Regs) -> Machine {
        Machine { regs }
    }

    fn exec(&mut self, instr: Instr) {
        let Instr(op, a, b, c) = instr;
        match op {
            Opcode::Addr => self.regs[c] = self.regs[a] + self.regs[b],
            Opcode::Addi => self.regs[c] = self.regs[a] + b as RegType,
            Opcode::Mulr => self.regs[c] = self.regs[a] * self.regs[b],
            Opcode::Muli => self.regs[c] = self.regs[a] * b as RegType,
            Opcode::Banr => self.regs[c] = self.regs[a] & self.regs[b],
            Opcode::Bani => self.regs[c] = self.regs[a] & b as RegType,
            Opcode::Borr => self.regs[c] = self.regs[a] | self.regs[b],
            Opcode::Bori => self.regs[c] = self.regs[a] | b as RegType,
            Opcode::Setr => self.regs[c] = self.regs[a],
            Opcode::Seti => self.regs[c] = a as RegType,
            Opcode::Gtir => self.regs[c] = (a as RegType > self.regs[b]) as RegType,
            Opcode::Gtri => self.regs[c] = (self.regs[a] > b as RegType) as RegType,
            Opcode::Gtrr => self.regs[c] = (self.regs[a] > self.regs[b]) as RegType,
            Opcode::Eqir => self.regs[c] = (a as RegType == self.regs[b]) as RegType,
            Opcode::Eqri => self.regs[c] = (self.regs[a] == b as RegType) as RegType,
            Opcode::Eqrr => self.regs[c] = (self.regs[a] == self.regs[b]) as RegType,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Sample {
    before: Regs,
    instr: Vec<usize>,
    after: Regs,
}

fn parse_samples(s: &str) -> Vec<Sample> {
    let re_before = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let re_after = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    let mut samples = Vec::new();
    let mut part1 = s.lines();
    while let Some(before) = part1.next() {
        let instr = part1.next().unwrap();
        let after = part1.next().unwrap();
        part1.next(); // skip blank line

        let mut before_regs = [0; 4];
        let bcaps = re_before.captures(before).unwrap();
        for i in 0..4 {
            before_regs[i] = bcaps[i+1].parse().unwrap()
        }

        let instr = instr.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut after_regs = [0; 4];
        let acaps = re_after.captures(after).unwrap();
        for i in 0..4 {
            after_regs[i] = acaps[i+1].parse().unwrap();
        }

        samples.push(Sample { before: before_regs, instr, after: after_regs });
    }
    samples
}

fn parse_instrs(s: &str, opcodes: &[Opcode]) -> Vec<Instr> {
    s.lines().map(|line| {
        let xs: Vec<_> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Instr(opcodes[xs[0]], xs[1], xs[2], xs[3])
    }).collect()
}

fn evaluate_samples(samples: &[Sample]) -> (u32, Vec<Vec<bool>>) {
    let mut like_three = 0;
    let mut valid_sets = vec![vec![true; 16]; 16];

    for sample in samples.iter() {
        let init_mach = Machine::new(sample.before);
        let instr = &sample.instr;
        let mut valid = 0;
        for o in 0..16 {
            let i = Instr(Opcode::from_usize(o), instr[1], instr[2], instr[3]);
            let mut mach = init_mach.clone();
            mach.exec(i);
            if mach.regs == sample.after {
                valid += 1;
            } else {
                valid_sets[instr[0]][o] = false;
            }
        }
        if valid >= 3 {
            like_three += 1;
        }
    }

    for i in 0..16 {
        println!("opcode {:2} could be {}", i, valid_sets[i].iter()
            .map(|&x| if x {'?'} else {'.'}).collect::<String>()
        );
    }

    (like_three, valid_sets)
}

// In general, this is a perfect matching problem on an unweighted bipartite graph.
// We can assume there is a unique solution that isn't too hard to deduce.
fn deduce_opcodes(valid_sets: &Vec<Vec<bool>>) -> Vec<Opcode> {
    let mut valid = valid_sets.clone();
    let mut opcodes = vec![None; 16];

    // Find an unassigned opcode with only one valid option.
    while let Some(i) = valid.iter().position(|set| set.iter().filter(|&&x| x).count() == 1) {
        // Assign it
        let j = valid[i].iter().position(|&x| x).unwrap();
        opcodes[i] = Some(Opcode::from_usize(j));
        for set in valid.iter_mut() {
            set[j] = false;
        }
    }

    opcodes.iter().map(|o| o.expect("unassigned opcode")).collect::<Vec<_>>()
}

fn solve(input: &str) -> (u32, u32) {
    let parts: Vec<_> = input.split("\n\n\n\n").collect();
    let samples = parts[0];
    let prog = parts[1];

    let samples = parse_samples(samples);
    let (like_three, valid_sets) = evaluate_samples(&samples);

    let opcodes = deduce_opcodes(&valid_sets);
    println!("{:?}", opcodes);

    let prog = parse_instrs(prog, &opcodes);
    let mut m = Machine::new([0;4]);
    for instr in prog {
        m.exec(instr);
    }

    (like_three, m.regs[0])
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
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
";

    #[test]
    fn parsing() {
        assert_eq!(parse_samples(EXAMPLE), vec![
            Sample { before: [3,2,1,1], instr: vec![9,2,1,2], after: [3,2,2,1] },
        ]);
    }

    #[test]
    fn example() {
        let samples = parse_samples(EXAMPLE);
        assert_eq!(1, evaluate_samples(&samples).0);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day16.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day16.txt"),
                   format!("{:?}", x));
    }
}
