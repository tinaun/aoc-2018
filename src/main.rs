#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

fn compare_case(b: u8, a: u8) -> bool {
    (a | 32) == (b | 32)
}

fn p1(next: &[u8]) -> usize {
    let mut next = next.to_vec();
    let mut last = next.len();

    loop {
        
        let mut skip = false;
        next = next.into_iter().chain(std::iter::once(0)).tuple_windows().filter_map(move |(a, b)| {
            if skip {
                skip = false;
                None
            } else if (a != b) && (a | 32) == (b | 32) {
                skip = true;
                None
            } else {
                Some(a)
            }
        }).collect();

        
        if last == next.len() {
            break next.len()
        } else {
            last = next.len();
        }

    }
}

fn main() {
    let demo = "abBA";//include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut count = 0;
    
    let mut next = input.trim().to_string().into_bytes();
    dbg!(p1(&next));

    let p2 = (b'a'..=b'z').map(|ch| {
        let next: Vec<_> = next.clone().into_iter().filter(|c| !compare_case(ch, *c)).collect();

        p1(&next)
    }).min();
    
    dbg!(p2);
}
