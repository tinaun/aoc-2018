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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Value::Tree => '|',
            Value::Yard => '#',
            Value::Open => '.',
        };

        write!(f, "{}", out)
    }
}

fn get(world: &Vec<Vec<Value>>, pos: (i32, i32)) -> Option<Value> {
    if let Some(l) = world.get(pos.1 as usize) {
        if let Some(j) = l.get(pos.0 as usize) {
            return Some(*j);
        }
    }

    None
}


fn find_neighbors(v: Value, (x, y): (i32, i32), w: &Vec<Vec<Value>>) -> Value {
    let n: Vec<Value> = vec![
        get(w, (x+1, y+1)),
        get(w, (x+1, y)),
        get(w, (x+1, y-1)),
        get(w, (x, y+1)),
        get(w, (x, y-1)),
        get(w, (x-1, y+1)),
        get(w, (x-1, y)),
        get(w, (x-1, y-1)),
    ].into_iter().filter_map(|x| x).collect();

    let tree_count = n.iter().filter(|&v| *v == Value::Tree).count();
    let yard_count = n.iter().filter(|&v| *v == Value::Yard).count();
    
    match v {
        Value::Tree => if yard_count >= 3 {
            Value::Yard
        } else {
            Value::Tree
        },
        Value::Yard => if tree_count >=1 && yard_count >= 1 {
            Value::Yard
        } else {
            Value::Open
        },
        Value::Open => if tree_count >= 3 {
            Value::Tree
        } else {
            Value::Open
        },
    }
}

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;

    let mut world = vec![vec![]];

    for (i, l) in input.lines().enumerate() {
        for c in l.trim().chars() {
            let out = match c {
                '|' => Value::Tree,
                '#' => Value::Yard,
                '.' => Value::Open,
                _ => continue,
            };

            world[i].push(out);
        }

        world.push(vec![]);
    }

    for y in world.iter() {
        for x in y.iter() {
            print!("{}", x);
        }

        println!();
    }
    let mut s = Vec::new();

    let limit = 1000;
    for i in 0..limit {
        let mut new_world = vec![vec![]];

        for (y, l) in world.iter().enumerate() {
            for (x, v) in l.iter().enumerate() {
                let next = find_neighbors(*v, (x as i32, y as i32), &world);

                new_world[y].push(next);
            }
            if !l.is_empty() {
                new_world.push(vec![]);
            }
        }

        world = new_world;

        // for y in world.iter() {
        //     for x in y.iter() {
        //         print!("{}", x);
        //     }

        //     println!();
        // }

        let mut wooded = 0;
        let mut yards = 0;
        for y in world.iter() {
            for &x in y.iter() {
                    if x == Value::Tree {
                        wooded += 1;
                    } else if x == Value::Yard {
                        yards += 1;
                    }
            }
        }

        if i == 9 {
            p1 = wooded * yards;
        }

        if i >= 602 && i < 631 {
            s.push((wooded, yards));
        }
    }

    let n = 1000000000;
    let (wooded, yards) = s[(n - 603) % 28];
    let p2 = wooded * yards;

    (p1, p2)
}
