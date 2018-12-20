#![allow(unused)]

mod prelude;
use self::prelude::*;

type Word = usize;


#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
struct Machine {
    regs: [Word; 6]
}

impl Machine {
    fn addr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] + self.regs[b]
    }

    fn addi(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] + b
    }
    
    fn mulr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] * self.regs[b]
    }

    fn muli(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] * b
    }

    fn banr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] & self.regs[b]
    }

    fn bani(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] & b
    }

    fn borr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] | self.regs[b]
    }

    fn bori(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] | b
    }

    fn setr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = self.regs[a] 
    }

    fn seti(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = a
    }

    fn gtir(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = if a > self.regs[b] {
            1
        } else {
            0
        }
    }

    fn gtri(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = if self.regs[a] > b {
            1
        } else {
            0
        }
    }

    fn gtrr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = if self.regs[a] > self.regs[b] {
            1
        } else {
            0
        }
    }

    fn eqir(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = if a == self.regs[b] {
            1
        } else {
            0
        }
    }

    fn eqri(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = if self.regs[a] == b {
            1
        } else {
            0
        }
    }

    fn eqrr(&mut self, a: Word, b: Word, c: Word) {
        self.regs[c] = if self.regs[a] == self.regs[b] {
            1
        } else {
            0
        }
    }

    fn test_everything(&self, test: Machine, a: Word, b: Word, c: Word) -> Vec<usize> {
        let mut tests = vec![self.clone(); 16];

        tests[0].addr(a, b, c);
        tests[1].addi(a, b, c);
        tests[2].mulr(a, b, c);
        tests[3].muli(a, b, c);
        tests[4].banr(a, b, c);
        tests[5].bani(a, b, c);
        tests[6].borr(a, b, c);
        tests[7].bori(a, b, c);
        tests[8].setr(a, b, c);
        tests[9].seti(a, b, c);
        tests[10].gtir(a, b, c);
        tests[11].gtri(a, b, c);
        tests[12].gtrr(a, b, c);
        tests[13].eqir(a, b, c);
        tests[14].eqri(a, b, c);
        tests[15].eqrr(a, b, c);

        tests.into_iter().enumerate().filter_map(|(i, m)| {
            if m == test {
                Some(i)
            } else {
                None
            }
        }).collect()
    }

    fn dispatch(&mut self, op: Word, a: Word, b: Word, c: Word) {
        match op {
            0 => self.addr(a, b, c),
            1 => self.addi(a, b, c),
            2 => self.mulr(a, b, c),
            3 => self.muli(a, b, c),
            4 => self.banr(a, b, c),
            5 => self.bani(a, b, c),
            6 => self.borr(a, b, c),
            7 => self.bori(a, b, c),
            8 => self.setr(a, b, c),
            9 => self.seti(a, b, c),
            10 => self.gtir(a, b, c),
            11 => self.gtri(a, b, c),
            12 => self.gtrr(a, b, c),
            13 => self.eqir(a, b, c),
            14 => self.eqri(a, b, c),
            15 => self.eqrr(a, b, c),
            _ => {},
        }
    }

    fn dispatch_instr(&mut self, op: &str, a: Word, b: Word, c: Word) {
        match op {
            "addr" => self.addr(a, b, c),
            "addi" => self.addi(a, b, c),
            "mulr" => self.mulr(a, b, c),
            "muli" => self.muli(a, b, c),
            "banr" => self.banr(a, b, c),
            "bani" => self.bani(a, b, c),
            "borr" => self.borr(a, b, c),
            "bori" => self.bori(a, b, c),
            "setr" => self.setr(a, b, c),
            "seti" => self.seti(a, b, c),
            "gtir" => self.gtir(a, b, c),
            "gtri" => self.gtri(a, b, c),
            "gtrr" => self.gtrr(a, b, c),
            "eqir" => self.eqir(a, b, c),
            "eqri" => self.eqri(a, b, c),
            "eqrr" => self.eqrr(a, b, c),
            _ => {},
        }
    }
    
}

type Instr = (String, Word, Word, Word);

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    let mut runner = Machine {
        regs: [0,0,0,0,0,0]
    };
    

    let mut prog = vec![];
    let mut ip = 0;
    let mut ip_reg: Option<Word> = None;

    for i in input.lines() {
        if let Ok(ip) = s!("#ip {}" <- i) {
            ip_reg = Some(ip);
            continue;
        }

        let i: Instr = scan(i).unwrap();

        prog.push(i);
    }
    
    let ip_reg = ip_reg.unwrap();
    let mut last_0 = 0;
    let mut count = 0;
    while ip < prog.len() {
        runner.regs[ip_reg] = ip;
        
        let (w, a, b, c) = prog.get(ip).unwrap();
        runner.dispatch_instr(w, *a, *b, *c);

        ip = runner.regs[ip_reg];
        ip += 1;
        count += 1;
        if runner.regs[0] > last_0 {
            last_0 = runner.regs[0];
            println!("{} ip={} {:?}", count, ip, runner.regs);
        }

        if runner.regs[0] > runner.regs[5] {
            //println!("{} ip={} {:?}", count, ip, runner.regs);
        }
        
    }

    p1 = runner.regs[0];

    for i in 1..=10551355 {
        if 10551355 % i == 0 {
            dbg!(i);
            p2 += i;
        }
    }

    (p1, p2)
}
