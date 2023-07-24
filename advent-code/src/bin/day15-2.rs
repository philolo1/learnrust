use std::cmp::{min, max};
use std::{fs};
use anyhow::{Result, Context};
use std::env;
use regex::Regex;

use std::collections::HashMap;

type Pair = (i32, i32);

fn dist(p1: &Pair, p2: &Pair) -> i32 {
   return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

struct Sensor {
    pos_x: i32,
    pos_y: i32,
    dist: i32
}

fn main() ->  Result<()> {
    println!("Day 11");
    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    // let lines: Vec<&str> = content.split("\n").filter(|x| x.len() > 0).collect();

    // dbg!(lines);

    let mut beacon_map = HashMap::new();



    // dbg!(&content);
    let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();

    let mut sensors: Vec<Sensor> = vec![];

    for c in re.captures_iter(&content) {
        let (_,arr) = c.extract::<4>();
        let arr = arr.map(|e| e.parse::<i32>().unwrap());
        let [x,y,x2,y2] = arr;
        beacon_map.insert((x2,y2), true);

        let beacon_dist = dist(&(x,y),&(x2,y2));

        sensors.push(Sensor {pos_x: x, pos_y: y, dist: beacon_dist});
    }

    let row = 4_000_000;


    for y in 0..=4_000_000 {
        let mut intervals = vec![];

        // create intervals
        for s in &sensors {
            let x_dist = s.dist - (s.pos_y - y).abs();

            if x_dist >= 0 {
                intervals.push((s.pos_x - x_dist, s.pos_x + x_dist));
            }
        }

        // sort intervals
        intervals.sort();

        let a = intervals[0].0;
        let mut b= intervals[0].1;

        for (c,d) in intervals {
            if c<=b+1 && d>=b {
                b = d;
            }
        }

        if a>0 || b <4_000_000 {
            println!("Interval: y: {y}, {a}-{b}");
            let mut res: i64 = b as i64 +1;
            res *= 4_000_000;
            res += y as i64;
            dbg!(res);
        }





    }







    Ok(())
}


