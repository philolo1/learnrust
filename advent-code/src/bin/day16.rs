use std::cmp::{max};
use std::rc::Rc;
use std::fs;
use anyhow::{Result, Context};
use std::env;
use regex::Regex;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    rate: i32,
    children: Vec<&'a str>
}

impl<'a> Node<'a> {
    fn new(name: &'a str, rate: i32, items: &'a str) -> Node<'a> {
        let children: Vec<&str> = items.split(", ").map(|x| x.trim()).collect();
        Node {
            name,
            rate,
            children
        }

    }
}

fn main() ->  Result<()> {
    println!("Day 11");

    let file_name = env::args().nth(1).context("One file is necessary")?;
    let content = fs::read_to_string(file_name)?;

    // dbg!(&content);
    // let re = Regex::new(r"Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)").unwrap();
    let re = Regex::new(r"Valve ([A-Z]+) has flow rate=([0-9]+);.*valve[s]? ([A-Z, ]+)").unwrap();


//     todo!();
    let mut node_map = HashMap::new();
    let mut nodes: Vec<Rc<Node<'_>>> = vec![];
     for c in re.captures_iter(&content) {
         let (_,arr) = c.extract::<3>();
         dbg!(arr);
         let name = arr[0];
         let rate: i32 = arr[1].parse().unwrap();
         let items = arr[2];

         if rate > 0 {
             println!("HELP {}", arr[0]);
         }

         let node = Rc::new(Node::new(name, rate, items));
         nodes.push(node.clone());

         node_map.insert(arr[0], node.clone());
     }

     let al = node_map.get("AA").unwrap();


     dbg!(al);

     let mut non_zero_nodes : Vec<&str>= nodes.iter().filter(|node| node.rate > 0).map(|node| node.name).collect();
     let mut non_zero_rates : Vec<i32> = vec![];
     non_zero_nodes.insert(0, "AA");
     dbg!(&non_zero_nodes);
     let mut dist_map: HashMap<&&str, HashMap<&str, i32>> = HashMap::new();

     for node in &non_zero_nodes {
         let mut node_hash_map: HashMap<&str, i32> = HashMap::new();

         let mut queue = VecDeque::new();
         queue.push_back((*node, 0));

         non_zero_rates.push(node_map.get(*node).unwrap().rate);

         while let Some((node_name, dist)) = queue.pop_front() {
             if None == node_hash_map.get(node_name) {
                 node_hash_map.insert(node_name, dist);
             }

             let children = node_map.get(node_name).unwrap();
             let iterator = children.children.iter();

             for neighbor in iterator {
                 if None  == node_hash_map.get(neighbor) {
                     queue.push_back((neighbor, dist + 1));
                 }
             }

         }

     //    println!("MAP for {} {:?}", node, node_hash_map);
         dist_map.insert(node, node_hash_map);
     }


    println!("SEARCH");
    let mut max_map = HashMap::new();
    let res = search(
        0,
        30,
        0,
        &dist_map,
        &non_zero_nodes,
        &mut max_map,
        &non_zero_rates
    );

    dbg!(res);


    Ok(())
}

fn search(
    current_pos: i32,
    minutes_left: i32,
    opened_pos: u32,
    dist: &HashMap<&&str, HashMap<&str, i32>>,
    non_zero_nodes: &Vec<&str>,
    max_map: &mut HashMap<(i32,i32,u32), i32>,
    non_zero_rates: &Vec<i32>,
    ) -> i32 {


    // check if already in the hashmap
    if let Some(val) = max_map.get(&(current_pos,minutes_left,opened_pos)) {
        return *val;
    }


    // no time left
    if minutes_left == 0 {
        return 0;
    }

    // calculate current rate
    let mut current_rate = 0;
    for i in 0..non_zero_rates.len() {
        if (opened_pos >> i) & 1 == 1 {
            current_rate += non_zero_rates[i];
        }
    }


    println!("Current pos: {current_pos} min: {minutes_left} current_rate:{current_rate}");

    // do nothing
    let mut res = minutes_left * current_rate;

    // check if can open vault
    if (opened_pos >> current_pos) & 1 == 0 {
        let new_open_pos = opened_pos | (1 << current_pos);
        println!("{new_open_pos} {} {}", current_pos, 1 << current_pos);
        let open_case = current_rate + search(current_pos, minutes_left - 1, new_open_pos, dist, non_zero_nodes, max_map, non_zero_rates);
        res = max(res, open_case);
    }

    // go to somewhere else
    for i in 0..non_zero_nodes.len() {
        if i == current_pos as usize {
            continue
        }
        let dist_map = dist.get(&non_zero_nodes[current_pos as usize]).unwrap();
        let go_dist = *dist_map.get(&non_zero_nodes[i]).unwrap();

        if go_dist > minutes_left {
            continue
        }

        let go_case = go_dist * current_rate + search(i as i32, minutes_left - go_dist, opened_pos, dist, non_zero_nodes, max_map, non_zero_rates);
        res = max(res, go_case);
    }



    max_map.insert((current_pos,minutes_left,opened_pos), res);

    return res;

}

