#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut count = 0;

    for c in input.lines() {
        let i: i32 = c.parse().unwrap();
        count += i;
    }
    

    println!("p1: {}", count);
}
