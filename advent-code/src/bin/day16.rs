use std::cmp::{min, max};
use std::rc::Rc;
use std::{fs};
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
    let mut nodes = vec![];
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
     non_zero_nodes.insert(0, "AA");
     dbg!(&non_zero_nodes);
     let mut dist_map = HashMap::new();

     for node in &non_zero_nodes {
         let mut node_hash_map: HashMap<&str, i32> = HashMap::new();

         let mut queue = VecDeque::new();
         queue.push_back((*node, 0));

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




    Ok(())
}

fn search(current_pos: i32, minutes_left: i32, opened_pos: i32) {
}

