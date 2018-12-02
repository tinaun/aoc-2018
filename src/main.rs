#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut count = 0;
    let mut repeats = Set::new();
    repeats.insert(count);

    for c in input.lines().cycle() {
        let i: i32 = c.parse().unwrap();
        count += i;
        if repeats.contains(&count) {
            dbg!(("p2", count));
            break;
        }
        repeats.insert(count);
    }
    

    //println!("p1: {}", count);
}
