#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

fn compare_case(b: u8, a: u8) -> bool {
    a == b || ((a as i32) - (b as i32)).abs() == 32
}

fn p1(next: &[u8]) -> usize {
    let mut next = next.to_vec();

    loop {
        //println!("{}", String::from_utf8_lossy(&next));
        let mut new = vec![];
        let mut i = 0;
            while i < next.len() - 1 {
                if ((next[i] as i32) - (next[i+1] as i32)).abs() == 32 {
                    i += 1;
                } else {
                    new.push(next[i]);
                }
                if i == next.len() - 2 {
                    new.push(next[i+1]);
                }
                i += 1;
            }

        //println!("{}", next.len());
        if next == new {
            break new.len()
        }
        
        next = new;
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
