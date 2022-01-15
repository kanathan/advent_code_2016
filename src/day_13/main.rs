use std::collections::{BinaryHeap, HashMap};
use std::time;

fn main() {
    println!("{}", get_fewest_moves(1362, State{x:31,y:39}));
    println!("{}", get_locations_count(1362, 50));
}


fn get_fewest_moves(key_val: u64, target: State) -> u64 {
    let cur_time = time::Instant::now();

    let mut frontier = BinaryHeap::new();
    let init_state = State{x:1,y:1};
    frontier.push(ScoredState{ state: init_state, cost: 0 });

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(init_state, init_state);
    cost_so_far.insert(init_state, 0);

    while !frontier.is_empty() {
        let ScoredState{state: current_state, cost: _} = frontier.pop().unwrap();
        let cur_cost = cost_so_far[&current_state];

        if current_state == target {
            println!("Solved in {} secs",cur_time.elapsed().as_secs_f32());
            return cur_cost
        }

        let new_cost = cur_cost + 1;
        for neighbor_state in current_state.get_valid_moves(key_val).iter() {
            if !cost_so_far.contains_key(neighbor_state) || new_cost < cost_so_far[neighbor_state] {
                cost_so_far.insert(*neighbor_state, new_cost);
                frontier.push(ScoredState{ state: *neighbor_state, cost: new_cost + neighbor_state.get_h_score(target) });
                came_from.insert(*neighbor_state, current_state);
            }
        }
    }

    unreachable!()
}


fn get_locations_count(key_val: u64, max_moves: u64) -> usize {
    let cur_time = time::Instant::now();

    let mut frontier = BinaryHeap::new();
    let init_state = State{x:1,y:1};
    frontier.push(ScoredState{ state: init_state, cost: 0 });

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(init_state, init_state);
    cost_so_far.insert(init_state, 0);

    while !frontier.is_empty() {
        let ScoredState{state: current_state, cost: _} = frontier.pop().unwrap();
        let cur_cost = cost_so_far[&current_state];

        let new_cost = cur_cost + 1;
        if new_cost > max_moves { continue }
        for neighbor_state in current_state.get_valid_moves(key_val).iter() {
            if !cost_so_far.contains_key(neighbor_state) || new_cost < cost_so_far[neighbor_state] {
                cost_so_far.insert(*neighbor_state, new_cost);
                frontier.push(ScoredState{ state: *neighbor_state, cost: new_cost });
                came_from.insert(*neighbor_state, current_state);
            }
        }
    }

    println!("Solved in {} secs",cur_time.elapsed().as_secs_f32());
    cost_so_far.len()
}


#[derive(Clone, PartialEq, Eq)]
struct ScoredState {
    state: State,
    cost: u64,
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


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: u64,
    y: u64,
}

impl State {
    fn get_valid_moves(&self, key_val: u64) -> Vec<State> {
        let mut valid_states = Vec::new();

        if self.x > 0 && valid_pos(self.x-1, self.y, key_val) {
            valid_states.push(State{x:self.x-1, y:self.y})
        }
        if self.y > 0 && valid_pos(self.x, self.y-1, key_val) {
            valid_states.push(State{x:self.x, y:self.y-1})
        }
        if valid_pos(self.x+1, self.y, key_val) {
            valid_states.push(State{x:self.x+1, y:self.y})
        }
        if valid_pos(self.x, self.y+1, key_val) {
            valid_states.push(State{x:self.x, y:self.y+1})
        }

        valid_states
    }

    fn get_h_score(&self, target: State) -> u64 {
        let x_diff = self.x.max(target.x) - self.x.min(target.x);
        let y_diff = self.y.max(target.y) - self.y.min(target.y);
        x_diff + y_diff
    }
}

fn valid_pos(x: u64, y: u64, key_val: u64) -> bool {
    let val = (x*x + 3*x + 2*x*y + y + y*y) + key_val;
    format!("{val:b}").chars().map(|c| {
        match c {
            '1' => 1,
            '0' => 0,
            _ => unreachable!()
        }
    })
    .sum::<i64>() % 2 == 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_fewest_moves(10, State{x:7, y:4}), 11);
    }

}