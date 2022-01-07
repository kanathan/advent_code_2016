

fn main() {
    let input = include_str!("input");
    println!("{}",get_code(input));
    println!("{}",get_code_part2(input));
}


fn get_code(input: &str) -> String {
    let mut code = String::new();

    let keypad = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9'],
    ];

    let mut row = 1;
    let mut col = 1;

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => if row > 0 {row -= 1},
                'D' => if row < 2 {row += 1},
                'L' => if col > 0 {col -= 1},
                'R' => if col < 2 {col += 1},
                _ => unreachable!()
            }
        }
        code.push(keypad[row][col]);
    }

    code
}

fn get_code_part2(input: &str) -> String {
    let mut code = String::new();

    let keypad = [
        ['.', '.', '1', '.', '.'],
        ['.', '2', '3', '4', '.'],
        ['5', '6', '7', '8', '9'],
        ['.', 'A', 'B', 'C', '.'],
        ['.', '.', 'D', '.', '.'],
    ];

    let mut row = 1;
    let mut col = 1;

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => if row > 0 && keypad[row-1][col] != '.' {row -= 1},
                'D' => if row < 4 && keypad[row+1][col] != '.' {row += 1},
                'L' => if col > 0 && keypad[row][col-1] != '.' {col -= 1},
                'R' => if col < 4 && keypad[row][col+1] != '.' {col += 1},
                _ => unreachable!()
            }
        }
        code.push(keypad[row][col]);
    }

    code
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "ULL\n\
    RRDDD\n\
    LURDL\n\
    UUUUD";

    #[test]
    fn test1() {
        println!("{}",get_code(INPUT))
    }

}