#![allow(unused)]

mod prelude;
use self::prelude::*;

fn find_max(mut input: &str, mut count: usize) -> usize {
    
    
    while input.len() > 0 {
        let mut next = &input[1..];

        match &input[..1] {
            "$" | "^" => {},
            "(" => {
                
                let mut depth = 1;
                let mut end = 1;
                let mut splits = vec![0];

                for (i,x) in next.chars().enumerate() {
                    match x {
                        '(' => depth += 1,
                        '|' if depth == 1 => {
                            splits.push(i);
                        },
                        ')' => {
                            if depth == 1 {
                                end = i;
                                splits.push(i);
                            } else {
                                depth -= 1;
                            }
                        },
                        _ => {},
                    }
                }

                let mut counts = vec![];
                for (start, end) in splits.into_iter().tuple_windows() {
                    //println!("{}", &next[start..end]);
                    counts.push(find_max(&next[start..end], 0));
                }

                if !counts.contains(&0) {
                    count += counts.into_iter().max().unwrap_or(0);
                }

                next = &next[end..];
                
            },
            "N" | "E" | "W" | "S" => count += 1,
            _ => {},
        }


        input = next;
        
    }

    count
}

fn flood_fill(mut input: &str, mut count: isize, world: &mut Map<(isize, isize), isize>, mut cursor: (isize, isize) )
    -> isize
{
    
    
    while input.len() > 0 {
        let mut next = &input[1..];

        match &input[..1] {
            "$" | "^" => {},
            "(" => {
                
                let mut depth = 1;
                let mut end = 1;
                let mut splits = vec![0];

                for (i,x) in next.chars().enumerate() {
                    match x {
                        '(' => depth += 1,
                        '|' if depth == 1 => {
                            splits.push(i);
                        },
                        ')' => {
                            if depth == 1 {
                                end = i;
                                splits.push(i);
                            } else {
                                depth -= 1;
                            }
                        },
                        _ => {},
                    }
                }

                let mut counts = vec![];
                for (start, end) in splits.into_iter().tuple_windows() {
                    //println!("{}", &next[start..end]);
                    counts.push(flood_fill(&next[start..end], count, world, cursor));
                }

                if !counts.contains(&count) {
                    count = counts.into_iter().max().unwrap_or(0);
                }

                next = &next[end..];
                
            },
            "N" => {
                count += 1;
                cursor.1 -= 1;
                if !world.contains_key(&cursor) {
                    world.insert(cursor, count);
                }

            },
            "E" => {
                count += 1;
                cursor.0 += 1;
                if !world.contains_key(&cursor) {
                    world.insert(cursor, count);
                }

            },
            "W" => {
                count += 1;
                cursor.0 -= 1;
                if !world.contains_key(&cursor) {
                    world.insert(cursor, count);
                }

            },
            "S" => {
                count += 1;
                cursor.1 += 1;
                if !world.contains_key(&cursor) {
                    world.insert(cursor, count);
                }

            },
            _ => {},
        }


        input = next;
        
    }

    count
}

pub fn advent() -> (impl Debug, impl Debug) {
    let demo = include_str!("../demo.txt");
    let input = include_str!("../input.txt");
    let mut p1 = 0;
    let mut p2 = 0;

    

    let p1 = find_max(input, 0);

    let mut world = Map::new();
    world.insert((0, 0), 0);

    let mut count = flood_fill(input, 0, &mut world, (0, 0));

    let img_data = &mut vec![0; 240 * 240]; 

    for y in 1..119 {
        for x in 1..119 {
            let mut pos = (x as isize - 60, y as isize - 60);

            if let Some(&current) = world.get(&pos) {
                pos.0 -= 1;
                let mut right = *world.get(&pos).unwrap_or(&0);
                pos.0 += 2;
                let mut left = *world.get(&pos).unwrap_or(&0);
                pos.1 -= 1; pos.0 -= 1;
                let mut top = *world.get(&pos).unwrap_or(&0);
                pos.1 += 2; 
                let mut bottom = *world.get(&pos).unwrap_or(&0);

                let v = 255 - ((current as f64 / (p1 as f64) * 255.0) as u8);
                
                img_data[(y*2+1) * 240 + (x*2+1)] = v;

                if (current - right).abs() == 1 {
                   img_data[(y*2+1) * 240 + (x*2)] = v;
                } 

                if (current - top).abs() == 1 {
                     img_data[(y*2) * 240 + (x*2+1)] = v;
                } 

                if (current - left).abs() == 1 {
                     img_data[(y*2+1) * 240 + (x*2+2)] = v;
                } 

                if (current - bottom).abs() == 1 {
                     img_data[(y*2+2) * 240 + (x*2+1)] = v;
                } 
            }
            
        }
    }

    use std::fs::File;
    let f = File::create("output.png").unwrap();
    let encoder = image::png::PNGEncoder::new(f);
    encoder.encode(&img_data, 240, 240, image::ColorType::Gray(8));

    let p2 = world.values().filter(|&&v| v >= 1000).count();

    (p1, p2)
}
