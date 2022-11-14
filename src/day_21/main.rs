use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let instrs_str = include_str!("input");
    println!("P1: {}", p1(instrs_str, "abcdefgh"));
    println!("P2: {}", p2(instrs_str, "fbgdceah"));
}


fn p1(instrs_str: &str, input: &str) -> String {
    let instrs = parse(instrs_str);
    let mut text: Vec<char> = input.chars().collect();
    for instr in instrs {
        performInstruction(&mut text, instr);
    }
    text.into_iter().collect::<String>()
}

fn p2(instrs_str: &str, input: &str) -> String {
    let instrs = rev_parse(instrs_str);
    let mut text: Vec<char> = input.chars().collect();
    for instr in instrs {
        performInstruction(&mut text, instr);
    }
    text.into_iter().collect::<String>()
}


lazy_static! {
    static ref REG_SWAP_POS: Regex = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    static ref REG_SWAP_LETTER: Regex = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
    static ref REG_ROT_L: Regex = Regex::new(r"rotate left (\d+) step").unwrap();
    static ref REG_ROT_R: Regex = Regex::new(r"rotate right (\d+) step").unwrap();
    static ref REG_ROT: Regex = Regex::new(r"rotate based on position of letter (\w)").unwrap();
    static ref REG_REV: Regex = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    static ref REG_MOV: Regex = Regex::new(r"move position (\d+) to position (\d+)").unwrap();
}


#[derive(Clone, Copy, Debug)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    Rotate(char),
    RevRotate(char),
    ReversePos(usize, usize),
    MovePos(usize, usize),
}


fn rotate_left(input: &mut Vec<char>, count: usize) {
    let len = input.len();
    for _ in 0..count {
        let temp = input[0];
        for i in 0..(len-1) {
            input[i] = input[i+1];
        }
        input[len-1] = temp;
    }
}

fn rotate_right(input: &mut Vec<char>, count: usize) {
    let len = input.len();
    for _ in 0..count {
        let temp = input[len-1];
        for i in (1..=(len-1)).rev() {
            input[i] = input[i-1];
        }
        input[0] = temp;
    }
}

fn rev_rotate(input: &mut Vec<char>, letter: char) {
    let idx = input.iter().position(|&c| c == letter).unwrap();
    let count = match idx {
        0 => 9,
        1 => 1,
        2 => 6,
        3 => 2,
        4 => 7,
        5 => 3,
        6 => 8,
        7 => 4,
        _ => unreachable!()
    };

    rotate_left(input, count);
}


fn performInstruction(input: &mut Vec<char>, instr: Instruction) {
    match instr {
        Instruction::SwapPos(x, y) => {
            input.swap(x, y);
        },
        Instruction::SwapLetter(a, b) => {
            let a_idx = input.iter().position(|&c| c == a).unwrap();
            let b_idx = input.iter().position(|&c| c == b).unwrap();
            input.swap(a_idx, b_idx);
        },
        Instruction::RotateLeft(x) => {
            rotate_left(input, x);
        },
        Instruction::RotateRight(x) => {
            rotate_right(input, x);
        },
        Instruction::Rotate(a) => {
            let idx = input.iter().position(|c| *c == a).unwrap();
            let count = if idx >= 4 {
                idx + 2
            } else {
                idx + 1
            };
            
            rotate_right(input, count);
        },
        Instruction::ReversePos(x, y) => {
            input[x..=y].reverse();
        },
        Instruction::MovePos(x, y) => {
            let val = input.remove(x);
            input.insert(y, val);
        },
        Instruction::RevRotate(a) => {
            rev_rotate(input, a);
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        instructions.push(
            if let Some(caps) = REG_SWAP_POS.captures(line) {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                Instruction::SwapPos(x, y)
            } else if let Some(caps) = REG_SWAP_LETTER.captures(line) {
                Instruction::SwapLetter(caps[1].chars().next().unwrap(), caps[2].chars().next().unwrap())
            } else if let Some(caps) = REG_ROT_L.captures(line) {
                Instruction::RotateLeft(caps[1].parse::<usize>().unwrap())
            } else if let Some(caps) = REG_ROT_R.captures(line) {
                Instruction::RotateRight(caps[1].parse::<usize>().unwrap())
            } else if let Some(caps) = REG_ROT.captures(line) {
                Instruction::Rotate(caps[1].chars().next().unwrap())
            } else if let Some(caps) = REG_REV.captures(line) {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                Instruction::ReversePos(x, y)
            } else if let Some(caps) = REG_MOV.captures(line) {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                Instruction::MovePos(x, y)
            } else {
                unreachable!("Invalid line: {line}")
            }
        );

    }

    instructions
}


fn rev_parse(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines().rev() {
        instructions.push(
            if let Some(caps) = REG_SWAP_POS.captures(line) {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                Instruction::SwapPos(x, y)
            } else if let Some(caps) = REG_SWAP_LETTER.captures(line) {
                Instruction::SwapLetter(caps[1].chars().next().unwrap(), caps[2].chars().next().unwrap())
            } else if let Some(caps) = REG_ROT_L.captures(line) {
                Instruction::RotateRight(caps[1].parse::<usize>().unwrap())
            } else if let Some(caps) = REG_ROT_R.captures(line) {
                Instruction::RotateLeft(caps[1].parse::<usize>().unwrap())
            } else if let Some(caps) = REG_ROT.captures(line) {
                Instruction::RevRotate(caps[1].chars().next().unwrap())
            } else if let Some(caps) = REG_REV.captures(line) {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                Instruction::ReversePos(x, y)
            } else if let Some(caps) = REG_MOV.captures(line) {
                let x = caps[1].parse::<usize>().unwrap();
                let y = caps[2].parse::<usize>().unwrap();
                Instruction::MovePos(y, x)
            } else {
                unreachable!("Invalid line: {line}")
            }
        );

    }

    instructions
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
"swap position 4 with position 0\n\
swap letter d with letter b\n\
reverse positions 0 through 4\n\
rotate left 1 step\n\
move position 1 to position 4\n\
move position 3 to position 0\n\
rotate based on position of letter b\n\
rotate based on position of letter d";

    #[test]
    fn test1() {
        assert_eq!("decab", p1(INPUT, "abcde"));
    }

}