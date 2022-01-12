use std::{collections::HashMap, hash::Hash};
use regex::Regex;

fn main() {
    let input = include_str!("input");
    let bot_map = parse_input(input);
    println!("{}",get_bot(bot_map.clone(), 61, 17));

    let bot_map = run_bots(bot_map);
    println!("{}",bot_map.into_iter().filter_map(|(k,v)| {
        match k {
            ID::Bot(_) => None,
            ID::Output(id) => {if id < 3 {Some(v.vals[0])} else {None}},
        }
    }).product::<u64>())
}

fn get_bot(mut bot_map: HashMap<ID,Bot>, v1: u64, v2: u64) -> u64 {
    let match_arr = vec![v1.min(v2), v1.max(v2)];
    let keys: Vec<ID> = bot_map.keys().cloned().collect();

    loop {
        let mut made_change = false;
        for id in keys.iter() {
            match id {
                ID::Bot(id_val) => {
                    if match_arr == bot_map.get(id).unwrap().vals {
                        return *id_val
                    }
                    let bot_in = bot_map.get_mut(id).unwrap();
                    if bot_in.vals.len() == 2 {
                        let high_val = bot_in.vals.pop().unwrap();
                        let high_id = bot_in.high_to.unwrap();
                        let low_val = bot_in.vals.pop().unwrap();
                        let low_id = bot_in.low_to.unwrap();
                        bot_map.get_mut(&high_id).unwrap().insert(high_val);
                        bot_map.get_mut(&low_id).unwrap().insert(low_val);
                        made_change = true;
                    }
                }
                ID::Output(_) => (),
            }
        }
        //print_bots(&bot_map);
        if !made_change { break }
    }

    0
}


fn run_bots(mut bot_map: HashMap<ID,Bot>) -> HashMap<ID,Bot> {
    let keys: Vec<ID> = bot_map.keys().cloned().collect();

    loop {
        let mut made_change = false;
        for id in keys.iter() {
            match id {
                ID::Bot(_) => {
                    let bot_in = bot_map.get_mut(id).unwrap();
                    if bot_in.vals.len() == 2 {
                        let high_val = bot_in.vals.pop().unwrap();
                        let high_id = bot_in.high_to.unwrap();
                        let low_val = bot_in.vals.pop().unwrap();
                        let low_id = bot_in.low_to.unwrap();
                        bot_map.get_mut(&high_id).unwrap().insert(high_val);
                        bot_map.get_mut(&low_id).unwrap().insert(low_val);
                        made_change = true;
                    }
                }
                ID::Output(_) => (),
            }
        }
        //print_bots(&bot_map);
        if !made_change { break }
    }

    bot_map
}

fn parse_input(input: &str) -> HashMap<ID,Bot> {
    let re_input = Regex::new(r"value (?P<val>\d+) goes to (?P<ty>\w+) (?P<id>\d+)").unwrap();
    let re_inst = Regex::new(
        r"bot (?P<in_id>\d+) gives low to (?P<lo_ty>\w+) (?P<lo_id>\d+) and high to (?P<hi_ty>\w+) (?P<hi_id>\d+)").unwrap();

    let mut containers: HashMap<ID, Bot> = HashMap::new();

    for line in input.lines() {
        if let Some(cap) = re_input.captures(line) {
            let val = cap.name("val").unwrap().as_str().parse::<u64>().unwrap();
            let ty = cap.name("ty").unwrap().as_str();
            let id = cap.name("id").unwrap().as_str().parse::<u64>().unwrap();
            match ty {
                "bot" => containers.entry(ID::Bot(id)).or_default().insert(val),
                "output" => containers.entry(ID::Output(id)).or_default().insert(val),
                _ => unreachable!()
            }
        } else if let Some(cap) = re_inst.captures(line) {
            let in_id = cap.name("in_id").unwrap().as_str().parse::<u64>().unwrap();
            let lo_ty = cap.name("lo_ty").unwrap().as_str();
            let lo_id = cap.name("lo_id").unwrap().as_str().parse::<u64>().unwrap();
            let hi_ty = cap.name("hi_ty").unwrap().as_str();
            let hi_id = cap.name("hi_id").unwrap().as_str().parse::<u64>().unwrap();
            
            let low = if lo_ty == "bot" { ID::Bot(lo_id) } else { ID::Output(lo_id) };
            let high = if hi_ty == "bot" { ID::Bot(hi_id) } else { ID::Output(hi_id) };
            containers.entry(ID::Bot(in_id)).or_default().set_outputs(low, high);
            containers.entry(low).or_default();
            containers.entry(high).or_default();
        }
    }

    containers
}

#[allow(dead_code)]
fn print_bots(bots: &HashMap<ID,Bot>) {
    for (id,bot) in bots {
        println!("{:?}: l={:?}, h={:?}, v={:?}",id,bot.low_to,bot.high_to,bot.vals);
    }
    println!()
}

#[derive(Default, Debug, Clone)]
struct Bot {
    low_to: Option<ID>,
    high_to: Option<ID>,
    vals: Vec<u64>,
}

impl Bot {
    fn insert(&mut self, val: u64) {
        self.vals.push(val);
        self.vals.sort_unstable();
    }

    fn set_outputs(&mut self, low: ID, high: ID) {
        self.low_to = Some(low);
        self.high_to = Some(high);
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum ID {
    Bot(u64),
    Output(u64),
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "value 5 goes to bot 2\n\
    bot 2 gives low to bot 1 and high to bot 0\n\
    value 3 goes to bot 1\n\
    bot 1 gives low to output 1 and high to bot 0\n\
    bot 0 gives low to output 2 and high to output 0\n\
    value 2 goes to bot 2";

    #[test]
    fn test1() {
        let bot_map = parse_input(INPUT);
        //print_bots(&bot_map);
        assert_eq!(get_bot(bot_map, 5, 2), 2);
    }

}