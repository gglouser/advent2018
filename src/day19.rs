use machine::*;

fn execute(ip: usize, prog: &[Instr]) -> RegType {
    let mut mach = Machine::new(ip);
    mach.run(&prog);
    mach.regs[0]
}

fn fast_part2(ip: usize, prog: &[Instr]) -> RegType {
    let mut mach = Machine::new(ip);
    mach.regs[0] = 1;
    while let Some(&instr) = prog.get(mach.regs[ip] as usize) {
        mach.exec(instr);
        mach.regs[ip] += 1;
        if mach.regs[ip] == 1 {
            break;
        }
    }
    let a = *mach.regs.iter().max().unwrap();
    let mut s = 0;
    let mut i = 1;
    while i*i < a {
        if a % i == 0 { s += i + a / i; }
        i += 1;
    }
    if i*i == a { s += i; }
    s
}

fn solve(input: &str) -> (RegType, RegType) {
    let (ip, prog) = parse_elfcode(input);
    (execute(ip, &prog), fast_part2(ip, &prog))
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
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";

    #[test]
    fn parsing() {
        assert_eq!(parse_elfcode(EXAMPLE), (0, vec![
            Instr(Opcode::Seti, 5, 0, 1),
            Instr(Opcode::Seti, 6, 0, 2),
            Instr(Opcode::Addi, 0, 1, 0),
            Instr(Opcode::Addr, 1, 2, 3),
            Instr(Opcode::Setr, 1, 0, 0),
            Instr(Opcode::Seti, 8, 0, 4),
            Instr(Opcode::Seti, 9, 0, 5),
        ]));
    }

    #[test]
    fn example() {
        let (ip, prog) = parse_elfcode(EXAMPLE);
        let r0 = execute(ip, &prog);
        assert_eq!(7, r0);
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day19.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day19.txt"),
                   format!("{:?}", x));
    }
}
