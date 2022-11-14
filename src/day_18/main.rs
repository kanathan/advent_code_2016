

fn main() {
    let input_str = include_str!("input");

    println!("Safe tiles: {}", get_safe_count(input_str, 40));

    println!("Safe tiles: {}", get_safe_count(input_str, 400000));

}


fn get_safe_count(input_str: &str, rows: usize) -> usize {
    let input = input_str.trim().chars().map(|c| {
        match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!("Invalid char {c}")
        }
    }).collect();

    let result = get_tilemap(input, rows);
    result.into_iter().flatten().filter(|&t| matches!(t, Tile::Safe)).count()
}


#[derive(Clone, Copy, Debug)]
enum Tile {
    Safe,
    Trap
}


fn get_tilemap(input: Vec<Tile>, rows: usize) -> Vec<Vec<Tile>> {
    let mut tilemap = vec![input];

    while tilemap.len() < rows {
        let new_row = calc_tiles(tilemap.last().unwrap());
        tilemap.push(new_row);
    }

    tilemap
}


fn calc_tiles(input: &[Tile]) -> Vec<Tile> {
    let mut new_row = Vec::with_capacity(input.len());

    for idx in 0..input.len() {
        let l = if idx == 0 { Tile::Safe } else { input[idx-1] };
        let r = if idx == input.len()-1 { Tile::Safe } else { input[idx+1] };
        let new_tile =
            match (l, input[idx], r) {
                (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
                (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe
            };
        new_row.push(new_tile);
    }

    new_row
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = ".^^.^.^^^^".chars().map(|c| {
            match c {
                '.' => Tile::Safe,
                '^' => Tile::Trap,
                _ => unreachable!("Invalid char {c}")
            }
        }).collect();

        let result = get_tilemap(input, 3);
        
        println!("{:?}", result);
    }

    #[test]
    fn test2() {
        assert_eq!(38, get_safe_count(".^^.^.^^^^", 10))
    }

}