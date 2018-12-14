#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
enum Track {
    LCurve,
    RCurve,
    Straight,
    Intersection,
    Nothing,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
enum Dir {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
struct Cart {
    state: Dir,
    facing: Facing,
}



impl Cart {
    fn turn_left(&mut self) {
        use self::Facing::*;

        self.facing = match self.facing {
            Left => Down,
            Right => Up,
            Up => Left,
            Down => Right,
        };
    }

    fn turn_right(&mut self) {
        use self::Facing::*;

        self.facing = match self.facing {
            Left => Up,
            Right => Down,
            Up => Right,
            Down => Left,
        };
    }

    fn move_forward(&self, mut loc: (usize, usize)) -> (usize, usize) {
        use self::Facing::*;

        match self.facing {
            Left => loc.1 -= 1,
            Right => loc.1 += 1,
            Up => loc.0 -= 1,
            Down => loc.0 += 1,
        };

        loc
    } 

    fn step(&mut self, track: Track, pos: (usize, usize)) -> (usize, usize) {
        use self::Track::*;
        match track {
            Straight => {},
            RCurve => {
                match self.facing {
                    Facing::Up | Facing::Down => self.turn_right(),
                    _ => self.turn_left(),
                }
            },
            LCurve => {
                match self.facing {
                    Facing::Up | Facing::Down => self.turn_left(),
                    _ => self.turn_right(),
                }
            },
            Intersection => {
                match self.state {
                    Dir::Left => {
                        self.turn_left();
                        self.state = Dir::Straight;
                    },
                    Dir::Straight => {
                        self.state = Dir::Right;
                    },
                        Dir::Right => {
                        self.turn_right();
                        self.state = Dir::Left;
                    },
                }
            }
            _ => panic!(),
        }

        self.move_forward(pos)
    }
}

fn tick(world: &[Vec<Track>], carts: &mut Map<(usize, usize), Cart>) -> bool {

    let mut new_carts = Map::new();
    let old_loc: Set<_> = carts.keys().cloned().collect();
    let mut to_skip = None;

    for (&loc, cart) in carts.into_iter() {
        //print!("{:?}", loc);
        if let Some(s) = to_skip {
            if loc == s {
                continue;
            }
        }

        let new_loc = cart.step(world[loc.0][loc.1], loc);
        if old_loc.contains(&new_loc) {
            println!("collision! {},{}", new_loc.1, new_loc.0);
            to_skip = Some(new_loc);
            continue;
        }

        if new_carts.insert(new_loc, *cart).is_some() {
            println!("collision! {},{}", new_loc.1, new_loc.0);
            new_carts.remove(&new_loc);
        }
    }

    if new_carts.len() == 1 {
        println!("{:?}", new_carts);
        return false;
    }
    

    *carts = new_carts;

    // for i in 50..71 {
    //     for j in 130..151 {
    //         let id = world[i][j];

    //         if let Some(cart) = carts.get(&(i, j)) {
    //             match cart.facing {
    //                 Facing::Left => print!("<"),
    //                 Facing::Right => print!(">"),
    //                 Facing::Up => print!("^"),
    //                 Facing::Down => print!("v"),
    //             }

    //             continue;
    //         }
    //         match id {
    //             Track::Intersection => print!("+"),
    //             Track::Straight => print!("."),
    //             Track::RCurve => print!("/"),
    //             Track::LCurve => print!("\\"),
    //             Track::Intersection => print!("+"),
    //             Track::Nothing => print!(" "),
    //         }
    //     }
    //     println!();
    // }
    

    true
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");

    let mut world = vec![vec![Track::Nothing; 200]; 200];
    let mut carts = Map::new();
    
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            world[y][x] = match c {
                '/' => Track::RCurve,
                '\\' => Track::LCurve,
                '|' | '-' => Track::Straight,
                '+' => Track::Intersection,
                c @ 'v' | c @ '^' | c @ '<' | c @ '>' => {
                    let facing = match c {  
                        'v' => Facing::Down,
                        '^' => Facing::Up,
                        '<' => Facing::Left,
                        '>' => Facing::Right,
                        _ => {return},
                    };

                    carts.insert((y, x), Cart{
                        state: Dir::Left,
                        facing
                    });

                    Track::Straight
                },
                _ => Track::Nothing,
            }
        }
    }

    //dbg!(&carts);

    while tick(&world, &mut carts) {

    }
}
