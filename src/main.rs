#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct Star {
    p: (i64, i64),
    v: (i64, i64),
}

impl Star {
    fn step(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
    }

    fn distance_from(&self, other: (i64, i64)) -> u64 {
        let x = self.p.0 - other.0;
        let y = self.p.1 - other.1;
        let s =  x*x + y*y;

        (s as f64).sqrt() as u64
    }
}

fn distance_from_center(starfield: &[Star]) -> u64 {
    let center = starfield.iter().fold((0, 0), |acc, next| {
        (acc.0 + next.p.0, acc.1 + next.p.1)
    });
    let count = starfield.len() as i64;
    let center = (center.0 / count, center.1 / count);

    let mut distance = starfield.iter().fold(0, |acc, star| {
        acc + star.distance_from(center)
    });

    distance
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    
    let mut starfield: Vec<Star> = input.lines().filter_map(|l| {
        s!("position=<{},{}> velocity=<{},{}>" <- l).ok()
    }).collect();

    let mut distance = distance_from_center(&starfield);
    
    for i in 0.. {
        for s in &mut starfield {
            s.step();
        }

        let new_dist = distance_from_center(&starfield);

        if distance > new_dist {
            distance = new_dist;
        } else {
            dbg!(i);
            for s in &mut starfield {
                s.p.0 -= s.v.0;
                s.p.1 -= s.v.1;
            }
            
            break;
        }
    }

    let min_x = starfield.iter().map(|s| s.p.0).min().unwrap();
    let max_x = starfield.iter().map(|s| s.p.0).max().unwrap();
    let min_y = starfield.iter().map(|s| s.p.1).min().unwrap();
    let max_y = starfield.iter().map(|s| s.p.1).max().unwrap();

    let x = (max_x - min_x) as usize + 1;
    let y = (max_y - min_y) as usize + 1;

    let mut display = vec![vec![' '; x]; y];

    for i in starfield {
        let y = (i.p.1 - min_y) as usize;
        let x = (i.p.0 - min_x) as usize;

        display[y][x] = 'x';
    }

    for y in display {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }

}
