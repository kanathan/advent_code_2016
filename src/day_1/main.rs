use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    println!("{}",get_distance(parse(input)));
    println!("{}",get_first_repeat_dist(parse(input)));
}


fn parse(input: &str) -> Vec<(Turn,i64)> {
    let mut directions = Vec::new();

    for cur_dir in input.split(',').map(|s| s.trim()) {
        let turn = 
            match cur_dir.chars().next().unwrap() {
                'L' => Turn::Left,
                'R' => Turn::Right,
                _ => unreachable!()
            };
        let dist = cur_dir[1..].parse::<i64>().unwrap();
        directions.push((turn, dist));
    }

    directions
}


fn get_distance(directions: Vec<(Turn,i64)>) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut dir = Dir::North;

    for (turn, dist) in directions.into_iter() {
        dir =
            match turn {
                Turn::Left => {
                    match dir {
                        Dir::North => Dir::West,
                        Dir::East => Dir::North,
                        Dir::South => Dir::East,
                        Dir::West => Dir::South,
                    }
                },
                Turn::Right => {
                    match dir {
                        Dir::North => Dir::East,
                        Dir::East => Dir::South,
                        Dir::South => Dir::West,
                        Dir::West => Dir::North,
                    }
                },
            };
        match dir {
            Dir::North => x += dist,
            Dir::East => y += dist,
            Dir::South => x -= dist,
            Dir::West => y -= dist,
        }
    }

    x.abs() + y.abs()
}


fn get_first_repeat_dist(directions: Vec<(Turn,i64)>) -> i64 {
    let mut visited = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut dir = Dir::North;
    visited.insert((x,y));

    for (turn, dist) in directions.into_iter() {
        dir =
            match turn {
                Turn::Left => {
                    match dir {
                        Dir::North => Dir::West,
                        Dir::East => Dir::North,
                        Dir::South => Dir::East,
                        Dir::West => Dir::South,
                    }
                },
                Turn::Right => {
                    match dir {
                        Dir::North => Dir::East,
                        Dir::East => Dir::South,
                        Dir::South => Dir::West,
                        Dir::West => Dir::North,
                    }
                },
            };
        for _ in 0..dist {
            match dir {
                Dir::North => x += 1,
                Dir::East => y += 1,
                Dir::South => x -= 1,
                Dir::West => y -= 1,
            }
            if !visited.insert((x,y)) {
                return x.abs() + y.abs()
            }
        }
        
    }

    0
}

enum Turn {
    Left,
    Right,
}

enum Dir {
    North,
    East,
    South,
    West,
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_distance(parse("R2, L3")), 5);
        assert_eq!(get_distance(parse("R2, R2, R2")), 2);
        assert_eq!(get_distance(parse("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(get_first_repeat_dist(parse("R8, R4, R4, R8")), 4);
    }
}