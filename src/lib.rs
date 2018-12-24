#![allow(unused)]

mod prelude;
use self::prelude::*;



#[derive(Debug, Copy, Clone, Deserialize, PartialEq)]
pub struct Nanobot {
    pos: (i64, i64, i64),
    range: i64,
}

impl Nanobot {
    fn in_range(&self, other: &Self) -> bool {
        self.in_range_pt(other.pos)
    }

    fn in_range_pt(&self, pos: (i64, i64, i64)) -> bool {
        let dist = (self.pos.0 - pos.0).abs()
        + (self.pos.1 - pos.1).abs() 
        + (self.pos.2 - pos.2).abs();
        //let dist = (dist as f64).sqrt();

        self.range >= dist
    }
}

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    let mut bots = vec![];

    for i in input.lines() {
        let bot: Nanobot = s!("pos=<{},{},{}>, r={}" <- i).unwrap();
        bots.push(bot);
    }

    let max = bots.iter().max_by_key(|b| b.range).unwrap();
    
    let mut total = 0;
    for other in &bots {
        if max.in_range(other) {
            total += 1;
        }
    }

    if total > p1 {
        p1 = total;
    }

    let mut x_map = Map::new();

    for b in &bots {
        let x_min = b.pos.0 + b.pos.1 + b.pos.2 - b.range;
        let x_max = b.pos.0 + b.pos.1 + b.pos.2 + b.range + 1;

        *x_map.entry(x_min).or_insert(0) += 1;
        *x_map.entry(x_max).or_insert(0) -= 1;
    }

    
    let mut running = 0;
    let mut max = 0;
    let mut max_start = 0;
    let mut max_end = 0;

    for (&pos, &v) in &x_map {
        running += v;
        if running > max {
            max = running;
            max_start = pos;
        }
    }

    println!("{}", max);

    let max_end = *x_map.keys().skip_while(|&&v| v <= max_start).next().unwrap();
    p2 = max_end - 1;

    (p1, p2)
}