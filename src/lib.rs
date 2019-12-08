#![allow(unused)]

mod prelude;
use self::prelude::*;

use regex::Regex;

#[derive(Debug, Copy, Clone, Deserialize, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Group {
    pts: [i32; 4]
}

impl Group {
    fn dist(&self, other: &Group) -> i32 {
        (self.pts[0] - other.pts[0]).abs() +
        (self.pts[1] - other.pts[1]).abs() +
        (self.pts[2] - other.pts[2]).abs() +
        (self.pts[3] - other.pts[3]).abs() 
    }
}


pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    let mut con: Vec<Set<Group>> = vec![];

    for i in input.lines() {
        let pts: Group = serde_scan::from_str_skipping(",", i).unwrap();
        let mut has_insert = false;
        for c in &mut con {
            for star in &*c {
                if star.dist(&pts) <= 3 {
                    has_insert = true;
                    break;
                }
            }

            if has_insert {
                c.insert(pts);
                break;
            }
        }

        if !has_insert {
            let mut c = Set::new();
            c.insert(pts);
            con.push(c);
        }
    }

    for _ in 0..1 {
        let mut to_merge = vec![];

        'main: for (i, x) in con.iter().enumerate() {
            for (j, y) in con.iter().enumerate() {
                for a in x {
                    for b in y {
                        if j != i && a.dist(b) <= 3 {
                            //println!("{:?} {:?} {}", a, b,a.dist(b));

                            if !to_merge.contains(&(j, i)) {
                                to_merge.push((i, j));
                            }
                            
                            continue 'main;
                        }
                    }
                }
            }
        }
        //println!("{:?}", to_merge);

        let mut new = vec![];
        let mut merged = Set::new();
        
        for (a, b) in to_merge {
            let mut m = con[a].clone();
            m.extend(con[b].clone());
            
            new.push(m);
            merged.insert(a);
            merged.insert(b); 
        }

        for (i, c) in con.iter().enumerate() {
            if !merged.contains(&i) {
                new.push(c.clone());
            }
        }

        p1 = new.len();
        dbg!(p1);
        con = new;
    }

    
    

    (p1, p2)
}