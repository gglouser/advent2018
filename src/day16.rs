use regex::Regex;
use machine::*;

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

#[derive(Debug, PartialEq)]
struct Sample {
    before: Vec<RegType>,
    instr: Vec<usize>,
    after: Vec<RegType>,
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

        let bcaps = re_before.captures(before).unwrap();
        let before_regs = (1..=4).map(|n| bcaps[n].parse().unwrap()).collect();

        let instr = instr.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let acaps = re_after.captures(after).unwrap();
        let after_regs = (1..=4).map(|n| acaps[n].parse().unwrap()).collect();

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

    let mut mach = Machine::new(5);
    for sample in samples.iter() {
        let instr = &sample.instr;
        let mut valid = 0;
        for o in 0..16 {
            mach.regs[..4].copy_from_slice(&sample.before);
            let i = Instr(Opcode::from_usize(o), instr[1], instr[2], instr[3]);
            mach.exec(i);
            if &mach.regs[..4] == sample.after.as_slice() {
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

fn solve(input: &str) -> (u32, RegType) {
    let parts: Vec<_> = input.split("\n\n\n\n").collect();
    let samples = parts[0];
    let prog = parts[1];

    let samples = parse_samples(samples);
    let (like_three, valid_sets) = evaluate_samples(&samples);

    let opcodes = deduce_opcodes(&valid_sets);
    println!("{:?}", opcodes);

    let prog = parse_instrs(prog, &opcodes);
    let mut m = Machine::new(5);
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
            Sample { before: vec![3,2,1,1], instr: vec![9,2,1,2], after: vec![3,2,2,1] },
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
