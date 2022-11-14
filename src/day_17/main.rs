use std::collections::VecDeque;
use md5;

const ROWS: i32 = 4;
const COLS: i32 = 4;

fn main() {
    let result = get_shortest_route("pslxynzg");
    println!("{}: {}", result, result.len());

    let result = get_longest_route("pslxynzg");
    println!("Longest route: {}", result);
}


struct PathInfo {
    row: i32,
    col: i32,
    path: String,
}


fn get_shortest_route(input: &str) -> String {
    let mut frontier = VecDeque::new();
    frontier.push_back(PathInfo {
        row: 0,
        col: 0,
        path: String::new(),
    });

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        if current.row == ROWS - 1 && current.col == COLS - 1 {
            return current.path
        }
        for next in get_neighbors(input, &current) {
            frontier.push_back(next);
        }
    }
    panic!("Unable to find path")
}

fn get_longest_route(input: &str) -> usize {
    let mut frontier = VecDeque::new();
    frontier.push_back(PathInfo {
        row: 0,
        col: 0,
        path: String::new(),
    });

    let mut longest_route = 0;

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        if current.row == ROWS - 1 && current.col == COLS - 1 {
            longest_route = longest_route.max(current.path.len());
            continue
        }
        for next in get_neighbors(input, &current) {
            frontier.push_back(next);
        }
    }
    
    longest_route
}

const PATHS: [(i32, i32, char); 4] = [(-1, 0, 'U'), (1, 0, 'D'), (0, -1, 'L'), (0, 1, 'R')];

fn get_neighbors(input: &str, current: &PathInfo) -> Vec<PathInfo> {
    let mut neighbors = Vec::new();

    let to_hash = format!("{}{}", input, current.path);
    let digest = md5::compute(to_hash);
    let hash = format!("{:x}", digest);

    // First four hash characters correspond to up, down, left, right
    // B-F => door is open

    for (hash_char, &(row_delta, col_delta, path_char)) in hash.chars().zip(PATHS.iter()) {
        let new_row = current.row + row_delta;
        let new_col = current.col + col_delta;
        if new_row < 0 || new_row >= ROWS || new_col < 0 || new_col >= COLS {
            continue
        }
        match hash_char {
            'b'..='f' => {
                let mut new_path = current.path.clone();
                new_path.push(path_char);

                let neighbor = PathInfo {
                    row: current.row + row_delta,
                    col: current.col + col_delta,
                    path: new_path
                };

                neighbors.push(neighbor);
            },
            _ => (),
        }
    }

    neighbors
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("DDRRRD", get_shortest_route("ihgpwlah"));
        assert_eq!("DDUDRLRRUDRD", get_shortest_route("kglvqrro"));
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", get_shortest_route("ulqzkmiv"));
    }

    #[test]
    fn test2() {
        assert_eq!(370, get_longest_route("ihgpwlah"));
        assert_eq!(492, get_longest_route("kglvqrro"));
        assert_eq!(830, get_longest_route("ulqzkmiv"));
    }

}