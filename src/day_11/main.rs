use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use lazy_static::lazy_static;
use itertools::Itertools;
use regex::Regex;
use std::time;

fn main() {
    let input = include_str!("input");
    let init_state = parse_input(input);
    println!("{}",get_fewest_moves(&init_state));

    let input = include_str!("input2");
    let init_state = parse_input(input);
    println!("{}",get_fewest_moves(&init_state));
}

lazy_static!{
    static ref MAT_MAP: HashMap<&'static str, usize> = [
        ("hydrogen", 0),
        ("lithium", 1),
        ("polonium", 2),
        ("thulium", 3),
        ("promethium", 4),
        ("ruthenium", 5),
        ("cobalt", 6),
        ("elerium", 7),
        ("dilithium", 8),
    ].iter().copied().collect();
}


fn get_fewest_moves(init_state: &State) -> u16 {
    let cur_time = time::Instant::now();
    let mut print_counter = 0;
    let mut cur_update_time = time::Instant::now();
    let mut frontier = BinaryHeap::new();
    frontier.push(ScoredState{ state: init_state.clone(), cost: 0 });

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(init_state.clone(), init_state.clone());
    cost_so_far.insert(init_state.clone(), 0);

    while !frontier.is_empty() {
        let ScoredState{state: current_state, cost: _} = frontier.pop().unwrap();
        let cur_cost = cost_so_far[&current_state];

        if current_state.is_complete() {
            println!("Solved in {} secs",cur_time.elapsed().as_secs_f32());
            return cur_cost
        }

        if print_counter < 50 {
            println!("{} secs: Currently at cost {}+{}={} with {} permutations",
                cur_time.elapsed().as_secs(),
                cur_cost,
                current_state.get_h_score(),
                cur_cost + current_state.get_h_score(),
                frontier.len());
            print_counter += 1;
        }
        if cur_update_time.elapsed().as_secs() > 5 {
            cur_update_time = time::Instant::now();
            println!("{} secs: Currently at cost {}+{}={} with {} permutations",
                cur_time.elapsed().as_secs(),
                cur_cost,
                current_state.get_h_score(),
                cur_cost + current_state.get_h_score(),
                frontier.len());
        }

        let new_cost = cur_cost + 1;
        for (neighbor_state, _) in current_state.get_valid_moves().iter() {
            if !cost_so_far.contains_key(neighbor_state) || new_cost < cost_so_far[neighbor_state] {
                cost_so_far.insert(neighbor_state.clone(), new_cost);
                frontier.push(ScoredState{ state: neighbor_state.clone(), cost: new_cost + neighbor_state.get_h_score() });
                came_from.insert(neighbor_state.clone(), current_state.clone());
            }
        }
        
        // Need to prune
        let mut best_score_map: HashMap<State,u16> = HashMap::new();
        for ScoredState { state, cost } in frontier.iter() {
            if !best_score_map.contains_key(state) || best_score_map[state] > *cost {
                best_score_map.insert(state.clone(), *cost);
            }
        }
        frontier = frontier.into_iter().filter(|ss| best_score_map[&ss.state] - 10 <= ss.cost).collect();
    }

    0
}

fn parse_input(input: &str) -> State {
    let re = Regex::new(r"(\w+)(?:-compatible)? ((?:microchip)|(?:generator))").unwrap();

    let mut floors = Vec::new();
    for line in input.lines() {
        let mut floor = Floor::new();
        for cap in re.captures_iter(line) {
            let mat_id = MAT_MAP[cap.get(1).unwrap().as_str()];
            match cap.get(2).unwrap().as_str() {
                "microchip" => floor.add_chip(mat_id),
                "generator" => floor.add_rtg(mat_id),
                _ => unreachable!(),
            }
            //println!("Found a {} {} with mat_id={}",cap.get(1).unwrap().as_str(),cap.get(2).unwrap().as_str(),mat_id);
        }
        floors.push(floor);
    }
    
    State {
        floors,
        elevator: 0,
    }
}

#[derive(Clone, PartialEq, Eq)]
struct ScoredState {
    state: State,
    cost: u16,
}

impl Ord for ScoredState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for ScoredState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq)]
struct State {
    floors: Vec<Floor>,
    elevator: usize,
}

impl State {
    fn is_complete(&self) -> bool {
        self.floors[0..(self.floors.len()-1)].iter().all(|f| f.is_empty())
    }

    fn get_valid_moves(&self) -> Vec<(State, u16)> {
        let mut valid_moves = Vec::new();

        let combinations = self.cur_floor().items.iter()
            .copied().combinations(1)
            .chain(self.cur_floor().items.iter().copied().combinations(2))
            .collect_vec();

        for combination in combinations.iter() {
            if !self.cur_floor().can_remove_ids(combination) { continue }
            if self.elevator < self.floors.len()-1 && self.floors[self.elevator+1].can_add_ids(combination) {
                let mut new_state = self.clone();
                for id in combination { new_state.cur_floor_mut().remove_id(*id) }
                new_state.elevator += 1;
                for id in combination { new_state.cur_floor_mut().add_id(*id) }
                valid_moves.push((new_state, 2 - combination.len() as u16));
            }
            if (0..self.elevator).map(|p| self.floors[p].is_empty()).all(|b| b) {
                // No need to move down
                continue
            }
            if self.elevator > 0 && self.floors[self.elevator-1].can_add_ids(combination) {
                let mut new_state = self.clone();
                for id in combination { new_state.cur_floor_mut().remove_id(*id) }
                new_state.elevator -= 1;
                for id in combination { new_state.cur_floor_mut().add_id(*id) }
                valid_moves.push((new_state, 1 + combination.len() as u16));
            }
        }

        valid_moves
    }

    fn cur_floor(&self) -> &Floor {
        &self.floors[self.elevator]
    }

    fn cur_floor_mut(&mut self) -> &mut Floor {
        &mut self.floors[self.elevator]
    }

    fn get_floor_pairs_vec(&self) -> Vec<usize> {
        let mut set_vals = vec![0;MAT_MAP.len()];
        for (floor_idx, floor) in self.floors.iter().enumerate() {
            for item in floor.items.iter() {
                let idx = *item % MAT_MAP.len();
                let val = if *item < MAT_MAP.len() { floor_idx } else { floor_idx * self.floors.len() };
                set_vals[idx] += val;
            }
        }
        set_vals.sort_unstable();
        set_vals
    }

    fn get_h_score(&self) -> u16 {
        let mut score = 0;
        let mut prev_items = 0;
        for floor in self.floors[0..(self.floors.len()-1)].iter() {
            let cur_items = floor.items.len() + prev_items;
            prev_items = cur_items;
            if cur_items == 0 { continue }
            if cur_items < 3 { score += 1 }
            else { score += cur_items * 2 - 3 }
        }
        score as u16
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_floor_pairs_vec().hash(state);
        self.elevator.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.get_floor_pairs_vec() == other.get_floor_pairs_vec() && self.elevator == other.elevator
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Floor {
    items: Vec<usize>,
}

impl Floor {
    fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn can_remove_ids(&self, ids: &[usize]) -> bool {
        let mut temp_items = self.items.clone();
        temp_items.retain(|v| !ids.contains(v));

        valid_item_list(&temp_items)
    }

    fn can_add_ids(&self, ids: &[usize]) -> bool {
        let mut temp_items = self.items.clone();
        temp_items.append(&mut ids.to_owned());

        valid_item_list(&temp_items)
    }

    fn add_id(&mut self, id: usize) {
        self.items.push(id);
        self.items.sort_unstable();
    }

    fn add_rtg(&mut self, mat_id: usize) {
        self.items.push(mat_id);
        self.items.sort_unstable();
    }

    fn add_chip(&mut self, mat_id: usize) {
        self.items.push(mat_id+MAT_MAP.len());
        self.items.sort_unstable();
    }

    fn remove_id(&mut self, id: usize) {
        self.items.retain(|i| *i != id);
    }
}

fn valid_item_list(items: &[usize]) -> bool {
    let mut unpowered_chips = false;
    let mut has_rtgs = false;
    for id in 0..MAT_MAP.len() {
        if !unpowered_chips && items.contains(&(id+MAT_MAP.len())) && !items.contains(&id) { unpowered_chips = true }
        if !has_rtgs && items.contains(&id) { has_rtgs = true }
        if unpowered_chips && has_rtgs { return false }
    }
    true
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.\n\
    The second floor contains a hydrogen generator.\n\
    The third floor contains a lithium generator.\n\
    The fourth floor contains nothing relevant.";

    #[test]
    fn test1() {
        assert_eq!(get_fewest_moves(&parse_input(INPUT)), 11);
    }

}