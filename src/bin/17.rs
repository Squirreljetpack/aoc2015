#![allow(unused_variables)]

use std::{collections::{HashMap}, fmt::{self}, rc::{Rc}};

advent_of_code::solution!(17);

macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!($($arg)*);
        }
    };
}

// (index, target), count
type CacheMap = HashMap<(u8, u64), u64>;

// fn brute_k(k: u64, arr: &[u64], target: u64) -> u64 {
//     todo!()
// }

fn brute(arr: &[u64], target: u64) -> Option<u64> {
    // target < 32 || 
    // _arr.len() < 5
    if target == 0 {
        return Some(1)
    } else if arr.len() == 1 {
        if target == arr[0] {
            return Some(1)
        } else {
            return Some(0)
        }
    };
    None
}

fn dfs(arr: &[u64], target: u64, cache: &mut CacheMap) -> u64 {
    if let Some(count) = brute(arr, target) {
        return count
    }
    if let Some(count) = cache.get(&(arr.len() as u8, target)) {
        return *count
    }
    
    // is it faster to go large -> small and abort left, or small -> large and full-abort?
    let left = if target >= arr[0] { dfs(&arr[1..], target - arr[0], cache) } else { 0 };
    let right = dfs(&arr[1..], target, cache);
    let count = left + right;
    
    cache.insert((arr.len() as u8, target), count);
    count
}

static mut TARGET: u64 = 150;

pub fn part_one(input: &str) -> Option<u64> {
    let mut arr: Vec<u64> = input
    .lines()
    .map(|s| s.parse::<u64>())
    .collect::<Result<Vec<u64>, _>>()
    .unwrap();
    // dbg!(&arr);
    arr.sort_by(|a, b| b.cmp(a));
    let mut cache = CacheMap::new();
    let count = unsafe { dfs(&arr,TARGET, &mut cache) };
    Some(count)
}

type CacheBestMap = HashMap<(u8, u64), ReturnNode>; // prefix (for keeping track), consumed, count

#[derive(Clone, Debug)]
pub struct NodeList {
    head: Link,
}

type Link = Option<Rc<ListNode>>;

#[derive(Debug)]
struct ListNode {
    elem: Node,
    next: Link
}

impl NodeList {
    pub fn new() -> Self {
        NodeList { head: None }
    }
    
    // prepend List is infeasible due to immutability so we can use this term
    // q: usage is a bit nicer with immutable (-> Self), but is there a performance impact?
    pub fn mut_prepend(&mut self, elem: Node) {
        let new_node = Rc::new(ListNode {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    
    pub fn head(&self) -> Option<&Node> {
        self.head.as_ref().map(|b| &b.elem)
    }
}

impl NodeList {
    pub fn iter(&self) -> NodeListIter<'_> {
        NodeListIter { current: self.head.as_deref() }
    }
}

pub struct NodeListIter<'a> {
    current: Option<&'a ListNode>,
}

impl<'a> Iterator for NodeListIter<'a> {
    type Item = &'a Node;
    
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some(&node.elem)
    }
}

impl NodeList {
    pub fn fmt_recursive(f: &mut fmt::Formatter<'_>, list: &Self, indent: usize) -> fmt::Result {
        for node in list.iter() {
            writeln!(f, "{}{}", "  ".repeat(indent), node.elem)?;
            Self::fmt_recursive(f, &node.children, indent + 1)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Node {
    elem: u64,
    children: NodeList,
}

#[derive(Clone, Debug)]
pub struct ReturnNode {
    depth: u64,
    children: NodeList,
}

impl ReturnNode {
    // represents no solution
    pub fn new() -> Self {
        Self {
            children: NodeList::new(),
            depth: 0,
        }
    }
    
    pub fn prepend_elem(self, elem: u64) -> Self {
        let child = Node {
            elem,
            children: self.children,
        };

        let mut children = NodeList::new();
        children.mut_prepend(child);
        
        Self {
            depth: self.depth + 1,
            children,
        }
    }
    
    pub fn prepend_elem_then_minmerge(self, elem: u64, mut other: Self) -> Self {
        let depth = self.depth + 1;

        if depth < other.depth || other.depth == 0 {
            return self.prepend_elem(elem);
        }

        if depth > other.depth {
            return other;
        }
        
        let child = Node {
            elem,  
            children: self.children,
        };
        
        other.children.mut_prepend(child);
        other
    }
    
    pub fn depth(&self) -> u64 {
        self.depth
    }

    pub fn leaves(&'_ self) -> LeafIter<'_> {
        LeafIter { stack: self.children.iter().collect() }
    }
}


impl fmt::Display for ReturnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        NodeList::fmt_recursive(f, &self.children, 0)
    }
}

pub struct LeafIter<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for LeafIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            for child in node.children.iter() {
                self.stack.push(child);
            }
            if node.children.head.is_none() {
                return Some(node);
            }
        }

        None
    }
}


fn dfs_tree(arr: &[u64], target: u64) -> u64 {
    let mut cache = CacheBestMap::new();
    let ret = dfs_tree_impl(arr, target, &mut cache);
    debug_eprintln!("{}", ret);

    ret.leaves().count() as u64
}
// potential optimizations:
// check remaining array sum occaisionally
    
fn dfs_tree_impl(arr: &[u64], target: u64, cache: &mut CacheBestMap) -> ReturnNode {
    debug_eprintln!("{:?} {}", arr, target);

    if arr.len() == 0 {
        return ReturnNode::new()
    }
    
    if let Some(ret) = cache.get(&(arr.len() as u8, target)) {
        debug_eprintln!("cache");
        return ret.clone()
    }
    
    let left = if target > arr[0] {
        debug_eprintln!("left");
        let ret = dfs_tree_impl(&arr[1..], target - arr[0], cache);
        if ret.depth > 0 {
            Some(ret)
        } else {
            None
        }
    } else if target == arr[0] { 
        Some(ReturnNode::new())
    } else {
        None
    };
    
    debug_eprintln!("right");
    let right = dfs_tree_impl(&arr[1..], target, cache);

    let ret = match left {
        Some(left_val) => left_val.prepend_elem_then_minmerge(arr[0], right),
        None => right,
    };

    debug_eprintln!("{:?} {} : {:?}", arr, target, ret);
    
    cache.insert((arr.len() as u8, target), ret.clone());
    ret.clone()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut arr: Vec<u64> = input
    .lines()
    .map(|s| s.parse::<u64>())
    .collect::<Result<Vec<u64>, _>>()
    .unwrap();
    arr.sort_by(|a, b| b.cmp(a));
    let count = unsafe { dfs_tree(&arr, TARGET) };
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part_one() {
        unsafe { TARGET = 25 };
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
    
    #[test]
    fn test_part_two() {
        unsafe { TARGET = 25 };
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
