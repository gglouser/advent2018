pub type RegType = u64;
pub type Val = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instr(pub Opcode, pub Val, pub Val, pub Val);

const NUM_REGS: usize = 6;

#[derive(Clone, Debug)]
pub struct Machine {
    pub regs: [RegType; NUM_REGS],
    ip: usize,
}

impl Machine {
    pub fn new(ip: usize) -> Machine {
        Machine { regs: [0; NUM_REGS], ip }
    }

    pub fn exec(&mut self, instr: Instr) {
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

    pub fn run(&mut self, prog: &[Instr]) {
        self.regs[self.ip] = 0;
        while let Some(&instr) = prog.get(self.regs[self.ip] as usize) {
            self.exec(instr);
            self.regs[self.ip] += 1;
        }
    }
}

pub fn parse_elfcode(s: &str) -> (usize, Vec<Instr>) {
    let mut lines = s.lines();
    let ip = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let instrs = lines.map(|line| {
        let mut i = line.split_whitespace();
        let op = match i.next().unwrap() {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            x => panic!("unrecognized instruction: {}", x),
        };
        let args: Vec<usize> = i.map(|x| x.parse().unwrap()).collect();
        Instr(op, args[0], args[1], args[2])
    }).collect();
    (ip, instrs)
}
