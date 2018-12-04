#![feature(dbg_macro)]
#![allow(unused)]

mod prelude;
use self::prelude::*;

#[derive(Debug, Deserialize, PartialOrd, PartialEq, Eq, Ord)]
struct TS {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

fn main() {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut count = 0;

    let mut sorted = input.lines().collect::<Vec<_>>();
    sorted.sort();

    let mut guards = Map::<u32, u32>::new();
    let mut precise_time = Map::<u32, Vec<u32>>::new();
    let mut sleep_start = 0;
    let mut current_guard = None;

    for l in sorted {

        if let [ts, rest] = l.split("] ").collect::<Vec<_>>().as_slice() {
            let ts: TS = s!("[{}-{}-{} {}:{}" <- ts).unwrap();
            let next: Option<u32> = serde_scan::from_str_skipping("Guard#", rest).ok();
            if next.is_some() {
                current_guard = next;
            }                

            match *rest {
                "falls asleep" => {
                    sleep_start = ts.minute;
                },
                "wakes up" => {
                    if let Some(guard) = current_guard {
                        *guards.entry(guard).or_insert(0) += ts.minute - sleep_start;
                        let time_vec = precise_time.entry(guard).or_insert(vec![0; 60]);

                        for val in &mut time_vec[(sleep_start as usize)..(ts.minute as usize)] {
                            *val += 1;
                        }

                        sleep_start = 0;
                    }
                },
                _ => {},
            }
            
        }
    }

    let (max, tot) = guards.iter().max_by_key(|&(a, b)| b).unwrap();

    let (min, _) = precise_time.get(max).unwrap().iter().enumerate().max_by_key(|&(min, &val)| val).unwrap();

    println!("p1: {}", *max * (min as u32));

    let guard = precise_time.iter().max_by_key(|&(guard, times)| {
        let max = times.iter().enumerate().max_by_key(|&(min, &val)| val).unwrap();
        max.1
    });

    if let Some((id, list)) = guard {
        let max = list.iter().enumerate().max_by_key(|&(min, &val)| val).unwrap();
        let p2 = *id * (max.0 as u32);
        dbg!(p2);
    }

    
}
