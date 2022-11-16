use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

fn main() {
    let input = include_str!("input");
    let map_data = parse(input);
    println!("{}", p1(&map_data));
    println!("{}", p2(&map_data));
}


fn p1(map_data: &MapData) -> usize {
    let distances = get_distances(&map_data);

    let mut min_distance = usize::MAX;
    for perm in (1..(distances.len() as u8)).permutations(distances.len()-1) {
        let mut cur = 0;
        let mut distance = 0;
        for next in perm {
            distance += distances[cur as usize].get(&next).unwrap();
            cur = next;
        }
        min_distance = min_distance.min(distance);
    }

    min_distance
}

fn p2(map_data: &MapData) -> usize {
    let distances = get_distances(&map_data);

    let mut min_distance = usize::MAX;
    for perm in (1..(distances.len() as u8)).permutations(distances.len()-1) {
        let mut cur = 0;
        let mut distance = 0;
        for next in perm {
            distance += distances[cur as usize].get(&next).unwrap();
            cur = next;
        }
        distance += distances[cur as usize].get(&0).unwrap();
        min_distance = min_distance.min(distance);
    }

    min_distance
}


#[derive(Clone, Copy, Debug)]
enum Tile {
    Wall,
    Path,
    Location(u8),
}


#[derive(Clone, Debug)]
struct MapData {
    map: Vec<Vec<Tile>>,
    goals: HashMap<u8, (usize, usize)>,
}


fn parse(input: &str) -> MapData {

    let mut map = Vec::new();
    let mut goals = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        map.push(
            line.chars().enumerate().map(|(x, ch)| {
                match ch {
                    '#' => Tile::Wall,
                    '.' => Tile::Path,
                    '0'..='9' => {
                        let goal_num = ch.to_digit(10).unwrap().try_into().unwrap();
                        goals.insert(goal_num, (x, y));
                        Tile::Location(goal_num)
                    },
                    _ => unreachable!("Invalid char {}", ch)
                }
            }).collect()
        );
    }

    MapData {
        map,
        goals
    }
}


fn get_distances(map_data: &MapData) -> Vec<HashMap<u8, usize>> {
    let mut distances = Vec::new();

    for origin_num in 0..(map_data.goals.len() as u8) {
        let &origin = map_data.goals.get(&origin_num).unwrap();
        let mut cur_distances = HashMap::new();

        let mut frontier_queue = VecDeque::new();
        frontier_queue.push_back((origin, 0));
        let mut visited = HashSet::new();

        while let Some(((x, y), distance)) = frontier_queue.pop_front() {
            if visited.contains(&(x,y)) { continue } else { visited.insert((x,y)); }

            for (x, y) in get_neighbors(&map_data, x, y) {
                if visited.contains(&(x,y)) { continue }
                match map_data.map[y][x] {
                    Tile::Wall => (),
                    Tile::Path => {
                        frontier_queue.push_back(((x, y), distance+1));
                    },
                    Tile::Location(goal) => {
                        if !cur_distances.contains_key(&goal) {
                            cur_distances.insert(goal, distance+1);
                        }
                        frontier_queue.push_back(((x, y), distance+1));
                    }
                }
            }
        }

        distances.push(cur_distances);
    }

    distances
}


fn get_neighbors(map_data: &MapData, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x-1, y));
    }
    if y > 0 {
        neighbors.push((x, y-1));
    }
    if x < map_data.map[0].len() - 1 {
        neighbors.push((x+1, y));
    }
    if y < map_data.map.len() - 1 {
        neighbors.push((x, y+1))
    }

    neighbors
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "###########\n\
    #0.1.....2#\n\
    #.#######.#\n\
    #4.......3#\n\
    ###########";

    #[test]
    fn test1() {
        let map_data = parse(INPUT);
        println!("{}",p1(&map_data));
    }

}