#![allow(unused)]

mod prelude;
use self::prelude::*;

use std::collections::VecDeque;

fn count_metadata(mut i: VecDeque<i32>, is_part_1: bool) -> (i32, VecDeque<i32>) {
    let mut nodes = i.pop_front().unwrap();
    let mut metadata = i.pop_front().unwrap();
    let mut sum = 0;
    let mut inner_nodes = vec![];

    while nodes > 0 {
        let (inner, next) = count_metadata(i, is_part_1);
        i = next;
        inner_nodes.push(inner);
        nodes -= 1;
    }

    if is_part_1 {
        sum = inner_nodes.iter().sum();
    }
    
    while metadata > 0 {
        let node = i.pop_front().unwrap() as usize;
        
        if inner_nodes.len() == 0 || is_part_1 {
            sum += node as i32;
        } else {
            sum += inner_nodes.get(node - 1).cloned().unwrap_or(0);
        }
        metadata -= 1;
    }

    (sum, i)
}

fn p1(input: &VecDeque<i32>) -> i32 {
    count_metadata(input.clone(), true).0
}

fn p2(input: &VecDeque<i32>) -> i32 {
    count_metadata(input.clone(), false).0
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    
    let entries = scan(input).unwrap();

    dbg!(p1(&entries));
    dbg!(p2(&entries));

}
