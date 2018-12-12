#![allow(unused)]

mod prelude;
use self::prelude::*;

fn power_level(x: u32, y: u32, id: u32) -> i32 {
    let rid = x + 10;
    let mut power = rid * y;
    power += id;
    power *= rid;
    power /= 100;
    power %= 10;

    power as i32 - 5
}

fn grid_size(s: usize, id: u32) -> (usize, usize, i32) {
    let mut grid = vec![0; 300 * 300];

    for y in 1..=300 {
        for x in 1..=300 {
            grid[300 * (y-1) + (x-1)] = power_level(x as u32, y as u32, id);
        }
    }

    let mut max = 0;
    let mut max_pos = (0, 0);
    
    for y in 1..300-s {
        for x in 1..300-s {
            let mut next = 0;
            for y in y..y+s {
                for x in x..x+s {
                    next += grid[300 * (y-1) + (x-1)];
                }
            }
            
            if next > max {
                max = next;
                max_pos = (x, y);
            }
        }
    }

    (max_pos.0, max_pos.1, max)
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    
    let input = 9435;
    assert_eq!(power_level(3, 5, 8), 4);
    
    println!("p1: {:?}", grid_size(3, input));

    let mut max = 0;
    let mut answer = (0, 0, 0);

    for size in 1..300 {
        let (new_x, new_y, new_max) = grid_size(size, input);
        if new_max > max {
            max = new_max;
            answer = (new_x, new_y, size);

            println!("p2: {:?}", answer);
        }

        //println!("{}", size);
    }
}
