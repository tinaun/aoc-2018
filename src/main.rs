#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;


fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");

    let mut map = Map::new();
    
    for l in input.lines() {
        let loc: (i32, i32) = s!("{}, {}" <- l).unwrap();
        map.insert(loc, 0);
    }

    let mut infinites = Set::<(i32, i32)>::new();
    let edges = [-400, 1000];

    for x in -400..=1000 {
        for y in -400..=1000 {
            let mut min_dist = 10_000_000;
            let mut closest = None;
            for &(a, b) in map.keys() {
                let dist = (a - x).abs() + (b - y).abs();
                if dist < min_dist {
                    min_dist = dist;
                    closest = Some((a, b));
                } else if dist == min_dist {
                    closest = None;
                }
            }

            if closest.is_some() {
                let closest = closest.unwrap();
                *map.entry(closest).or_insert(0) += 1;
                if edges.contains(&x) || edges.contains(&y) {
                    infinites.insert(closest);
                }
            }
        } 
    }

    let max = map.iter().filter(|x| !infinites.contains(&x.0)).max_by_key(|&(a, b)| b);

    dbg!(max);

    let limit = 10_000;
    let heuristic = 10_000 / map.len();
    let mut count = 0;

    for x in -100..=600 {
        for y in -100..=600 {
            let mut sum = 0;
            for &(a, b) in map.keys() {
                sum += (a - x).abs() + (b - y).abs();
            }

            if sum < limit {
                count += 1;
            }
        }
    }

    dbg!(count);
}
