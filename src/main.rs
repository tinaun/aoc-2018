#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
enum Track {
    
}

fn main() {
    let input_2 = [5,0,5,9,6,1];
    let input = 505961;

    let first = 3;
    let second = 7;
    let mut list = vec![first, second];

    let mut e1 = 0;
    let mut e2 = 1;

    loop {
        let v1 = list[e1];
        let v2 = list[e2];
        let mut new = v1 + v2;
        if new >= 10 {
            list.push(new / 10);
            new %= 10;
        }

        if list.len() == input + 10 {
            println!("p1: {:?}", &list[list.len()-10..]);
        }

        if list.len() > 6 && &list[list.len()-6..] == input_2 {
            break;
        }

        list.push(new);
        e1 += 1 + v1;
        e2 += 1 + v2;

        e2 %= list.len();
        e1 %= list.len();

        if list.len() > 6 && &list[list.len()-6..] == input_2 {
            break;
        }

        if list.len() == input + 10 {
            println!("p1: {:?}", &list[list.len()-10..]);
        }
    }

    println!("p2: {}", list.len() - 6);
}
