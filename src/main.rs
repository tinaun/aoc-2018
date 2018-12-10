#![allow(unused)]

mod prelude;
use self::prelude::*;

use doubly::DoublyLinkedList as LL;

fn solve(players: usize, lines: u64) -> u64 {
    let mut players = vec![0; players];
    let mut p_idx = 0;

    let mut circle = LL::new();
    circle.push_front(0);
    let mut idx = 0;

    for i in 1..=lines {
        if i % 23 == 0 {
            idx += (circle.len() - 7);

            idx %= circle.len();

            let next = circle.remove(idx+1);
            players[p_idx] += i + next;

            //println!("added {} + {} {} to {}", next, i, (next+i), p_idx); 
            
            idx %= circle.len();
        } else {
            idx += 2;
            idx %= circle.len();

            circle.insert(idx+1, i);
            
        }

        p_idx += 1;
        p_idx %= players.len();
        //println!("{:?}", circle);        
    }

    players.into_iter().max().unwrap_or(0)
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    

    println!("p1: {}", solve(428, 70825));
    println!("p2: {}", solve(428, 7082500));
    
    

}
