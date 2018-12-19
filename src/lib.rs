#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Value {
    Tree,
    Yard,
    Open,
}

impl Value {

}

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    let mut world = ();

    for (i, l) in input.lines().enumerate() {
        
    }

    (p1, p2)
}
