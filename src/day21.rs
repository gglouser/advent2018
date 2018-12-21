use std::collections::HashSet;
use machine::*;

fn is_r0_test(mach: &Machine, instr: Instr) -> Option<RegType> {
    match instr {
        Instr(Opcode::Eqir, v, 0, _) | Instr(Opcode::Eqri, 0, v, _) => {
            Some(v as RegType)
        },
        Instr(Opcode::Eqrr, 0, r, _) | Instr(Opcode::Eqrr, r, 0, _) => {
            Some(mach.regs[r])
        },
        _ => None,
    }
}

// Correct but slow when last is true (part 2).
fn watch_r0(ip: usize, prog: &[Instr], last: bool) -> RegType {
    let mut mach = Machine::new(ip);
    let mut values = HashSet::new();
    let mut last_v = 0;
    while let Some(&instr) = prog.get(mach.regs[ip] as usize) {
        if let Some(v) = is_r0_test(&mach, instr) {
            if last {
                if !values.insert(v) {
                    return last_v;
                }
                last_v = v;
            } else {
                return v;
            }
        }
        mach.exec(instr);
        mach.regs[ip] += 1;
    }
    panic!("register 0 never checked");
}

fn fast_part2() -> u64  {
    let mut values = HashSet::new();
    let mut a = 0u64;
    let mut last_a = 0;
    loop {
        let mut b = a | 0x10000;
        a = 0xc154d6;
        while b > 0 {
            a = (((a + (b & 0xff)) & 0xffffff) * 65899) & 0xffffff;
            b /= 256;
        }
        if !values.insert(a) {
            break;
        }
        last_a = a;
    }
    last_a
}

fn solve(_input: &str) -> (RegType, RegType) {
    let (ip, prog) = parse_elfcode(_input);
    let part1 = watch_r0(ip, &prog, false);
    // let part2 = watch_r0(ip, &prog, true);
    let part2 = fast_part2();
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
    fn eqir_r0() {
        let (ip, prog) = parse_elfcode("\
#ip 5
eqir 99 0 1
");
        assert_eq!(99, watch_r0(ip, &prog, false));
    }

    #[cfg(feature="test_real_input")]
    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day21.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day21.txt"),
                   format!("{:?}", x));
    }
}
