#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Debug, PartialEq)] 
struct Rule {
    spread: Vec<bool>,
    out: bool,
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    
    let mut lines = input.lines();
    let mut grid = Map::new();
    let mut rules = vec![];
    //lines.next();
    let inital = lines.next().unwrap();
    lines.next();
    for (i, ch) in (&inital[15..]).chars().enumerate() {
        match ch {
            '#' => { grid.insert(i as i32, true); },
            _ => {},
        }
    }

    for l in lines {
        let (spread, out): (String, char) = s!("{} => {}" <- l).unwrap();

        let spread: Vec<bool> = spread.chars().map(|ch| {
            match ch {
                '.' => false,
                '#' => true,
                _ => false,
            }
        }).collect();

        let out = match out {
                '.' => false,
                '#' => true,
                _ => false,
            };

        rules.push(Rule {
            spread,
            out
        })
    }

    for gens in 0..20 {
        let mut next_grid = Map::new();
        let min = grid.keys().min().cloned().unwrap();
        let max = grid.keys().max().cloned().unwrap();

        for idx in min-2 ..= max+2 {
            let a = grid.get(&(idx-2)).cloned().unwrap_or(false);
            let b = grid.get(&(idx-1)).cloned().unwrap_or(false);
            let c = grid.get(&(idx)).cloned().unwrap_or(false);
            let d = grid.get(&(idx+1)).cloned().unwrap_or(false);
            let e = grid.get(&(idx+2)).cloned().unwrap_or(false);
            let rule = [a, b, c, d, e];

            for r in &rules {
                if r.spread == &rule {
                    next_grid.insert(idx, r.out);
                }
            }


        }

        grid = next_grid;
        
        
    }
        let sum = grid.iter().filter_map(|(idx, &pot)| if pot { Some(idx) } else { None }).sum::<i32>();
        dbg!(sum);

        let f = "###.##.##.#.....###.##.#..........###.##.##.##.##.##.##.##.##.#......###.##.##.##.##.##.##.##.##.##.##.##.##.##.##.##.#.......###.##.##.##.#";
        let offset = 40;

        let mut try_sum = 0;
        for (i, ch) in f.chars().enumerate() {
            if ch == '#' {
                try_sum += (50_000_000_000 - offset) + i
            }
        }

        println!("p2: {}", try_sum);

    

    

    
}
