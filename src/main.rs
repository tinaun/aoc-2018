#![allow(unused)]

mod prelude;
use self::prelude::*;


#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
struct Entity {
    ty: EntityTy,
    attack: i32,
    hp: i32,
}

impl Entity {
    fn goblin() -> Self {
        Entity {
            ty: EntityTy::Goblin,
            attack: 3,
            hp: 200,
        }
    }
    fn elf(attack: usize) -> Self {
        Entity {
            ty: EntityTy::Elf,
            attack: 3,
            hp: 200,
        }
    }
    fn wall() -> Self {
        Entity {
            ty: EntityTy::Wall,
            attack: -1,
            hp: 100_000,
        }
    }
    fn nothing() -> Self {
        Entity {
            ty: EntityTy::Nothing,
            attack: -1,
            hp: 100_000,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)] 
enum EntityTy {
    Goblin,
    Elf,
    Wall,
    Nothing,
}

impl Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            EntityTy::Elf => write!(f, "E"),
            EntityTy::Goblin => write!(f, "G"),
            EntityTy::Wall => write!(f, "#"),
            EntityTy::Nothing => write!(f, "."),
        }
    }
}

fn simulate(mut world: Map<(usize, usize), Entity>, elf_power: i32) -> (i32, i32, bool) {
    let mut turns = 0;
    for (_, elf) in world.iter_mut().filter(|(_, e)| e.ty == EntityTy::Elf) {
        elf.attack = elf_power;
    }
    let mut dead_elves = false;


    'game: loop {
        let targets: Set<_> = world.iter()
            .filter(|(&pos, e)| e.ty == EntityTy::Goblin || e.ty == EntityTy::Elf)
            .map(|(&pos, e)| pos).collect();

        for mut e in targets {
            let mut current_entity = match world.remove(&e) {
                Some(e) => e,
                None => continue,
            };

            let mut other_targets: Set<_> = world.iter()
            .filter(|(&pos, e)| e.ty == EntityTy::Goblin || e.ty == EntityTy::Elf)
            .map(|(&pos, e)| pos).collect();

            let mut valid_targets: Set<_> = other_targets
                .into_iter()
                .filter(|pos| world[pos].ty != current_entity.ty)
                .collect();

            if valid_targets.len() == 0 {
                world.insert(e, current_entity);
                break 'game;
            }

            //spots in range to targets
            valid_targets = valid_targets
                .into_iter()
                .flat_map(|(x, y)| vec![(x+1, y),(x-1, y),(x, y+1),(x, y-1)])
                .filter(|pos| world.get(pos).is_none())
                .collect();

            let mut current = None;

            // pathfind
            let mut used = Set::new();
            let mut next = Set::new();
            next.insert(e);
            'target: loop {
                if valid_targets.len() == 0 || next.len() == 0 {
                    break;
                }


                if let Some(new_target) = valid_targets.intersection(&next).min_by_key(|pos| pos.0) {
                    current = Some(*new_target);
                    break 'target;
                }

                used.extend(next.clone());
                next = next
                    .into_iter()
                    .flat_map(|(x, y)| vec![(x+1, y),(x-1, y),(x, y+1),(x, y-1)])
                    .filter(|pos| world.get(pos).is_none() && !used.contains(pos))
                    .collect();
            }
            

            //dbg!(current);
            if let Some(pos) = current {
                let mut distance_set = Map::new();
                distance_set.insert(pos, 0);

                while !distance_set.contains_key(&e) {
                    let next: Map<_,_> = distance_set
                        .iter()
                        .flat_map(|((x, y), i)| {
                            let (x, y, i) = (*x, *y, *i);

                            vec![((x+1, y), i+1),((x-1, y), i+1),((x, y+1), i+1),((x, y-1), i+1)]
                        })
                        .filter(|(pos, _)| world.get(pos).is_none() && !distance_set.contains_key(pos))
                        .collect();

                    distance_set.extend(next);
                }

                let neighbors = [(e.0+1, e.1),(e.0-1, e.1),(e.0, e.1+1),(e.0, e.1-1)];

                // for x in distance_set.iter().filter(|(pos, _)| neighbors.contains(pos)) {
                //     println!("{:?}", x);
                // }

                let next = distance_set.into_iter()
                .filter(|(pos, _)| neighbors.contains(pos))
                .min_by_key(|(pos, _)| pos.0);

                if let Some(next) = next {
                    //println!("target {:?}, next {:?}", current, next.0);
                    
                    e = next.0;
                }
            }


            //entity is done moving
            world.insert(e, current_entity);
            
            let attack = world.get(&e).unwrap().attack;
            let ty = world.get(&e).unwrap().ty;

            //select attacker
            let mut min_hp = 10_000;
            let mut target = None;
            for t in &[(e.0-1, e.1),(e.0, e.1-1),(e.0, e.1+1),(e.0+1, e.1)] {
                let hp = world.get(t).unwrap_or(&Entity::nothing()).hp;
                let en_ty = world.get(t).unwrap_or(&Entity::nothing()).ty;
                if hp < min_hp && en_ty != ty {
                    min_hp = hp;
                    target = Some(*t);
                }
            }

            let mut died = None;
            if let Some(t) = target {
                world.get_mut(&t).unwrap().hp -= attack;
                //println!("attack! {:?} attacked {:?} for 3 damage", e, t);

                if world.get_mut(&t).unwrap().hp < 0 {
                    died = Some(t);
                }
            }

            if let Some(t) = died {
                if let EntityTy::Elf = world.get(&t).unwrap_or(&Entity::nothing()).ty {
                    //println!("elves are over!!");
                    //return (0, 0);
                    dead_elves = true;
                }
                //println!("{:?} died", t);
                world.remove(&t);
            }

            
        }

        // for y in 0..10 {
        //     for x in 0..10 {
        //         print!("{}", world.get(&(y, x)).unwrap_or(&Entity::nothing()));
        //     }
        //     println!();
        // }

        // let total_hp = world.iter()
        //     .filter(|(&pos, e)| e.ty == EntityTy::Goblin || e.ty == EntityTy::Elf)
        //     .fold(0, |acc, (_, elem)| acc + dbg!(elem.hp));

        // dbg!(total_hp);

        turns += 1;
    }

    let total_hp = world.iter()
            .filter(|(&pos, e)| e.ty == EntityTy::Goblin || e.ty == EntityTy::Elf)
            .fold(0, |acc, (_, elem)| acc + elem.hp);

    dbg!(total_hp);

    (turns, total_hp, dead_elves)
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");

    let mut entities = Map::new();
    
    for (y, l) in input.lines().enumerate() {
        for (x, ch) in l.chars().enumerate() {
            let e = match ch {
                '#' => Entity::wall(),
                'G' => Entity::goblin(),
                'E' => Entity::elf(3),
                _ => continue,
            };

            entities.insert((y, x), e);
        }
    }

    let (rounds, hp, _) = simulate(entities.clone(), 3);

    println!("p1: {}", rounds * hp);

    for power in 3.. {
        if let (rounds, hp, false) = simulate(entities.clone(), power) {
            println!("p2: {}", rounds * hp);
            break;
        }
    }

}
