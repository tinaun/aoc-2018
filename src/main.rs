#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Debug, Deserialize)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut count = 0;
    let mut claims = vec![vec![0; 1000]; 1000];

    for l in input.lines() {
        let c: Claim = s!("#{} @ {},{}: {}x{}" <- l).unwrap();

        //let c = dbg!(c);

        for x in c.x..(c.x+c.width) {
            for y in c.y..(c.y+c.height) {
                claims[y as usize][x as usize] += 1;
            }
        }

    }

    for x in &claims {
        for y in x {
            if *y > 1 {
                count += 1;
            }
            //print!("{}", y);
        }
        //println!("")
    }

    dbg!(count);

    'recheck: for l in input.lines() {
        let c: Claim = s!("#{} @ {},{}: {}x{}" <- l).unwrap();

        //dbg!(c);

        for x in c.x..(c.x+c.width) {
            for y in c.y..(c.y+c.height) {
                if claims[y as usize][x as usize] != 1 {
                    continue 'recheck;
                }
            }
        }

        dbg!(c.id);
    }    
}
