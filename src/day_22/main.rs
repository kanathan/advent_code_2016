use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use itertools::Itertools;

lazy_static! {
    static ref RE: Regex = Regex::new(r"-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T").unwrap();
}


fn main() {
    let input = include_str!("input");
    println!("P1: {}", p1(input));
    println!();
    p2(input);
}


fn p1(input: &str) -> usize {
    let nodes = parse(input);
    let mut count = 0;

    for pair in nodes.values().permutations(2) {
        let a = pair[0];
        let b = pair[1];

        if a.used == 0 { continue }
        if a.used > b.avail { continue }
        count += 1;
    }

    count
}


fn p2(input: &str) {
    // Visualize map, and calculate moves manually
    // For our input this is:
    // 27 moves to get empty node to upper left corner
    // Another 5 moves to move goal over to the left
    // Times 33 times to get to final position
    // Or 192 moves
    let nodes = parse(input);

    for y in 0.. {
        for x in 0.. {
            if let Some(node) = nodes.get(&(x, y)) {
                if node.used as f32 / (node.size as f32) < 0.4 {
                    print!("_")
                }
                else if node.used as f32 / (node.size as f32) < 0.9 {
                    print!(".")
                }
                else {
                    print!("#")
                }
            } else {
                break
            }
        }
        if !nodes.contains_key(&(0, y)) {
            break
        }
        println!()
    }
}


struct Node {
    size: u32,
    used: u32,
    avail: u32,
}


fn parse(input: &str) -> HashMap<(u32, u32), Node> {
    let mut lines = input.lines();
    lines.next();
    lines.next();

    let mut nodes = HashMap::new();

    for line in lines {
        let caps = RE.captures(line).unwrap();
        let x = caps[1].parse::<u32>().unwrap();
        let y = caps[2].parse::<u32>().unwrap();
        let size = caps[3].parse::<u32>().unwrap();
        let used = caps[4].parse::<u32>().unwrap();
        let avail = caps[5].parse::<u32>().unwrap();
        nodes.insert((x, y), Node { size, used, avail });
    }

    nodes
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test1() {

    }

}