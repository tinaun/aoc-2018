#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut count = 0;

    let mut twice = 0;
    let mut thrice = 0;
    for l in input.lines() {
        let mut t = Map::new();
        for x in l.chars() {
            *t.entry(x).or_insert(0) += 1;
        }

        for (k, v) in &t {
            if *v == 2 {
                twice += 1;
                break;
            }
        }

        for (k, v) in &t {
            if *v == 3 {
                thrice += 1;
                break;
            }
        }
    }

    println!("p1: {}", twice * thrice);

    fn check(a: &str, b: &str) -> Option<String> {
        if a == b {
            return None;
        }
        let mut once = None;
        let mut res = String::new();
        for (i, (x, y)) in a.chars().zip(b.chars()).enumerate() {
            if x != y && once.is_none() {
                once = Some(());
            } else if x != y {
                return None;
            } else {
                res.push(x);
            }
        }

        once.map(|_| res)
    }


    for (x, y) in input.lines().tuple_combinations() {
        if let Some(ans) = check(&x, y) {
            println!("p2: {}", ans);
        }
    }


}
