#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Deserialize, Debug, Copy, Clone)]
struct Vein {
    major: char,
    pos: u32,
    minor: char,
    mpos: (u32, u32),
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Value {
    Spring,
    Clay,
    Flowing,
    Stopped,
    Sand,
}

impl Value {
    fn can_flow(&self) -> bool {
        match self {
            Value::Flowing | Value::Sand => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Value::Spring => '+',
            Value::Clay => '#',
            Value::Flowing => '|',
            Value::Stopped => '~',
            Value::Sand => '.',
        };

        write!(f, "{}", out)
    }
}

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");

    let mut world = Map::new();
    world.insert((500, 0), Value::Spring);
    
    let mut low_y = 100;
    let mut high_y = 0;
    for l in input.lines() {
        let next: Vein = s!("{}={}, {}={}..{}" <- l).unwrap();
        match next.major {
            'x' => {
                let x = next.pos;

                for y in next.mpos.0..=next.mpos.1 {
                    world.insert((x, y), Value::Clay);
                }

                if next.mpos.1 > high_y {
                    high_y = next.mpos.1;
                }

                if next.mpos.0 < low_y {
                    low_y = next.mpos.0;
                }
            },
            'y' => {
                let y = next.pos;

                for x in next.mpos.0..=next.mpos.1 {
                    world.insert((x, y), Value::Clay);
                }

                if y > high_y {
                    high_y = y;
                }

                if y < low_y {
                    low_y = y;
                }
            },
            _ => {},
        }
    }

    let mut active_set: Vec<_> = world.iter()
            .filter(|(p, &v)| v == Value::Flowing || v == Value::Spring)
            .map(|(p, v)| p).cloned().collect();
    
    while !active_set.is_empty() {
        
        let mut next = vec![];

        'set: for (x, mut y) in active_set {
            while *world.entry((x, y+1)).or_insert(Value::Flowing) == Value::Flowing {
                y += 1;
                if y > high_y {
                    continue 'set;
                }
            }
            
            let mut xmin = x;
                    let mut xmax = x;
                    let mut lgap = false;
                    let mut rgap = false;
                    let mut finished = false;

                    while !finished {
                        finished = true;
                        if !lgap && *world.get(&(xmin-1,y)).unwrap_or(&Value::Sand) != Value::Clay {
                            xmin -= 1;
                            finished = false;
                            if world.get(&(xmin,y+1)).unwrap_or(&Value::Sand).can_flow() {
                                lgap = true;
                            }
                        }

                        if !rgap && *world.get(&(xmax+1,y)).unwrap_or(&Value::Sand) != Value::Clay {
                            xmax += 1;
                            finished = false;
                            if world.get(&(xmax,y+1)).unwrap_or(&Value::Sand).can_flow() {
                                rgap = true;
                            }
                        }
                    }

                    for x in xmin..=xmax {
                        world.insert((x, y), if lgap || rgap {
                            Value::Flowing
                        } else {
                            Value::Stopped
                        });
                    }

                    if lgap && rgap {
                        next.push((xmin, y));
                        next.push((xmax, y));
                    } else if lgap {
                        next.push((xmin, y));
                    } else if rgap {
                        next.push((xmax, y));
                    } else {
                        for x in xmin..=xmax {
                            if world.get(&(x, y-1)).is_some() {
                                next.push((x, y-1));
                            }
                        }
                    }
        }

        active_set = next;
        dbg!(active_set.len());

        // for y in 40..=60 {
        //     for x in 484..=517 {
        //         print!("{}", world.get(&(x, y)).unwrap_or(&Value::Sand));
        //     }
        //     println!();
        // }

    }

    let mut p1 = 0;
    for (k, v) in &world {
        if (k.1 >= low_y && k.1 <= high_y) && (*v == Value::Stopped || *v == Value::Flowing) {
            p1 += 1;
        }
    }

    let mut p2 = 0;
    for (k, v) in &world {
        if (k.1 >= low_y && k.1 <= high_y) && (*v == Value::Stopped) {
            p2 += 1;
        }
    }


    (p1, p2)
}
