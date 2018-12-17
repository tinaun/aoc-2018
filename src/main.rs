#![allow(unused)]

mod prelude;
use self::prelude::*;

type Word = usize;


#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
struct Machine {
    regs: [Word; 4]
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
    
}


fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");

    let input = input.lines().collect::<Vec<_>>();
    let mut chunks = input.chunks(4).into_iter();

    let mut p1 = 0;
    let mut values: Map<usize, Set<usize>> = Map::new();
    while let Some([before, i, after, _]) = chunks.next() {
        let before: Machine = match s!("Before: [{}, {}, {}, {}]" <- before) {
            Ok(b) => b,
            _ => break,
        };

        let [n, a, b, c]: [Word; 4] = scan(i).unwrap();
        let result: Machine = serde_scan::from_str_skipping("After:[,]", after).unwrap();

        let results = before.test_everything(result, a, b, c);
        if results.len() >= 3 {
            p1 += 1;
        }

        values.entry(n).or_insert_with(|| Set::new()).extend(results);
    }

    dbg!(p1);
    let mut f = Map::new();


    while !values.is_empty() {
        let mut next = None;
        for (&i, v) in &values {
            if v.len() == 1 {
                next = v.iter().next().cloned();
                f.insert(i, next.unwrap());
            }
        }

        for (_, v) in &mut values {
            if let Some(n) = next {
                v.remove(&n);
            }
        }

        values = values.into_iter().filter(|(_, v)| !v.is_empty()).collect();
    }

    let input = include_str!("../input-2.txt");
    let mut runner = Machine {
        regs: [0; 4]
    };

    for i in input.lines() {
        let [n, a, b, c]: [Word; 4] = scan(i).unwrap();

        runner.dispatch(f[&n], a, b, c);
    }
    
    println!("p2: {}", runner.regs[0]);

}
