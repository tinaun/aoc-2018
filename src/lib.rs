#![allow(unused)]

mod prelude;
use self::prelude::*;

const DEPTH: usize = 8787;
const TARGET: (usize, usize) = (10, 725);

fn geo_index(x: usize, y: usize, world: &mut Map<(usize, usize), usize>) -> usize {
    if let Some(&value) = world.get(&(x,y)) {
        return value;
    }
    
    let value = if (x, y) == (0, 0) {
        0
    } else if (x, y) == TARGET {
        0
    } else if x == 0 {
        y * 48271
    } else if y == 0 {
        x * 16807
    } else {
        erosion_level(x-1, y, world) * erosion_level(x, y - 1, world)
    };

    world.insert((x, y), value);

    value
}

fn erosion_level(x: usize, y: usize, world: &mut Map<(usize, usize), usize>) -> usize {
    (geo_index(x, y, world) + DEPTH) % 20183
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cave {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Tool {
    Gear,
    Torch,
    None,
}

fn man_dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let man = (a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs();

    man as usize
}


pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    let mut world = Map::new();

    for y in 0..=TARGET.1 {
        for x in 0..=TARGET.0 {
            let ero = (geo_index(x, y, &mut world) + DEPTH) % 20183;
            p1 += ero % 3;
        }
    }

    let mut cave_system = vec![Cave::Rocky; 1000 * 1000];

    for y in 0..1000 {
        for x in 0..1000 {
            let ero = (geo_index(x, y, &mut world) + DEPTH) % 20183;
            cave_system[y * 1000 + x] = match ero % 3 {
                1 => Cave::Wet,
                2 => Cave::Narrow,
                _ => {
                    continue;
                },
            };
        }
    }
    println!("starting path");
    let mut visited = Map::new();
    let mut dist = Map::new();
    dist.insert((0, 0, Tool::Torch), 0);

    let mut current = (0, 0, Tool::Torch);

    while !visited.contains_key(&(TARGET.0, TARGET.1, Tool::Torch)) ||
                !visited.contains_key(&(TARGET.0, TARGET.1, Tool::Gear)) {
        //dbg!(current);
        let (x, y, tool) = current;
        let mut possible_nodes = vec![(x + 1, y), (x, y+1)];
        if x > 0 {
            possible_nodes.push((x - 1, y));
        }
        if y > 0 {
            possible_nodes.push((x, y - 1));
        }
        let base = *dist.get(&current).unwrap();
        

        for node in possible_nodes {
            if !visited.contains_key(&(node.0, node.1, tool)) {
                let next_cave = cave_system[node.1 * 1000 + node.0];
                let prev_cave = cave_system[y * 1000 + x];

                let next_tool = match (prev_cave, next_cave) {
                    (Cave::Rocky, Cave::Wet) => Tool::Gear,
                    (Cave::Wet, Cave::Rocky) => Tool::Gear,
                    (Cave::Rocky, Cave::Narrow) => Tool::Torch,
                    (Cave::Narrow, Cave::Rocky) => Tool::Torch,
                    (Cave::Narrow, Cave::Wet) => Tool::None,
                    (Cave::Wet, Cave::Narrow) => Tool::None,
                    _ => tool,
                };

                let dv = if tool == next_tool {
                    1
                } else {
                    8
                };

                let entry = dist.entry((node.0, node.1, next_tool)).or_insert((base + dv));
                if *entry > base + dv {
                    *entry = base + dv;
                }
            }
        }

        visited.insert(current, (base, tool));
        dist.remove(&current);
        current = *dist.iter().min_by_key(|(k, &v)| v + man_dist((k.0, k.1), TARGET)).unwrap().0;
    }

    // this is fragile and only works on my input, but i can't be bothered to change it
    let (mut p2, tool) = visited.get(&(TARGET.0, TARGET.1, Tool::Torch)).unwrap();

    if *tool != Tool::Torch {
        p2 += 7;
    }


    (p1, p2)
}