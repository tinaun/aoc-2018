#![allow(unused)]

mod prelude;
use self::prelude::*;

use regex::Regex;

#[derive(Debug, Clone, Deserialize, Default)]
struct Group {
    is_bad: bool,
    units: u32,
    hp: u32,
    weakness: Vec<String>,
    immune: Vec<String>,
    attack: u32,
    ty: String,
    init: u32,
}

impl Group {
    fn power(&self) -> u32 {
        self.units * self.attack
    }

    fn team(&self) -> &'static str {
        if self.is_bad {
            "infection"
        } else {
            "immune system"
        }
    }

    fn is_weak(&self, to: &String) -> bool {
        self.weakness.contains(to)
    }

    fn is_immune(&self, to: &String) -> bool {
        self.immune.contains(to)
    }

    fn take_damage_from(&mut self, other: Group) -> bool {
        if !self.is_immune(&other.ty) {
            let damage = if self.is_weak(&other.ty) {
                other.power() * 2
            } else {
                other.power()
            };

            let losses = damage / self.hp;
            //println!("killed {} units!", losses);

            if losses >= self.units {
                self.units = 0;
                return true;
            } else {
                self.units -= losses;
            }
        }

        false
    }
}

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    let re = Regex::new(r"(\d+) units each with (\d+) hit points (?:\((.+?)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();

    let mut is_bad = false;

    let mut army = vec![];
    let boost = 34;

    for i in input.lines() {
        if i.starts_with("Immune System:") {
            is_bad = false;
        } else if i.starts_with("Infection:") {
            is_bad = true;
        }

        for cap in re.captures_iter(i.trim()) {
            let mut g = Group::default();
            g.units = cap[1].parse().unwrap();
            g.hp = cap[2].parse().unwrap();
            g.attack = cap[4].parse().unwrap();
            g.ty = cap[5].to_owned();
            g.init = cap[6].parse().unwrap();
            g.is_bad = is_bad;

            if !g.is_bad {
                g.attack += boost;
            }

            if let Some(s) = cap.get(3) {
                for unit in s.as_str().split("; ") {
                    if unit.starts_with("weak to") {
                        g.weakness = unit[8..].split(", ").map(|s| s.to_owned()).collect();
                    }
                    if unit.starts_with("immune to") {
                        g.immune = unit[10..].split(", ").map(|s| s.to_owned()).collect();
                    }
                }
            }

            army.push(g);
        }
    }
    
    let mut last_units = 0;
    loop {
        let mut attacks = Map::new();

        army.sort_by_key(|a| (0 - a.power() as i32, 0 - a.init as i32));
        
        for (i, a) in army.iter().enumerate() {
            let mut max_power = 0;
            let mut max_damage = 0;
            let mut max_initiative = 0;
            let mut max_id = None;
            for (j, d) in army.iter().enumerate() {
                if a.is_bad == d.is_bad || attacks.contains_key(&j) {
                    continue
                }

                if d.is_immune(&a.ty) {
                    continue;
                }

                let damage = if d.is_weak(&a.ty) {
                    a.power() * 2
                } else {
                    a.power()
                };

                //println!("{} group {} would attack def group {} {} damage", a.team(), i, j, damage);

                if damage > max_damage {
                    max_damage = damage;
                    max_power = d.power();
                    max_initiative = d.init;
                    max_id = Some(j);
                } else if damage == max_damage && d.power() > max_power {
                    max_power = d.power();
                    max_initiative = d.init;
                    max_id = Some(j);
                } else if damage == max_damage && d.power() == max_power && d.init > max_initiative {
                    max_initiative = d.init;
                    max_id = Some(j);
                } 

            }

            if let Some(id) = max_id {
                attacks.insert(id, (i, a.init));
            }
        }

        let mut attacks = attacks.into_iter().map(|(d, (a, id))| (id, (a, d)) ).collect_vec();
        attacks.sort_by_key(|(i, _)| 1000 - i);

        let mut to_remove = vec![];

        for (_, (att, def)) in attacks {
            //print!("{} {} ", army[att].units, army[def].units);
            let attacker = army[att].clone();
            if army[def].take_damage_from(attacker) {
                to_remove.push(def)
            }
        }

        army = army.into_iter().enumerate().filter(|(i, _)| !to_remove.contains(i)).map(|(_, e)| e).collect();

        let total_units = army.iter().fold(0, |acc, e| acc + e.units);
        if last_units == total_units {
            if army[0].is_bad {
                println!("bad end!");
            } else {
                println!("good end!");
            }
            p1 = total_units;
            break;
        } else {
            last_units = total_units;
        }
    }
    

    (p1, p2)
}