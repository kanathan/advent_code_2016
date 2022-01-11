use regex::Regex;
use ndarray::Array2;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input");
    println!("{}", get_pixel_count(input));
}


fn get_pixel_count(input: &str) -> u64 {
    let instrs = parse_input(input);
    let mut screen = Array2::default([6,50]);

    for inst in instrs {
        step(inst, &mut screen);
    }

    println!("{}", format_screen(&screen));

    screen.into_iter().map(|b| if b {1} else {0}).sum::<u64>()
}


fn step(inst: Inst, screen: &mut Array2<bool>) {
    match inst {
        Inst::Rect((width,height)) => {
            for x in 0..height {
                for y in 0..width {
                    *screen.get_mut([x,y]).unwrap() = true;
                }
            }
        },
        Inst::RotateCol((col_idx,offset)) => {
            let mut queue = VecDeque::new();
            for row in 0..screen.shape()[0] {
                queue.push_back(*screen.get([row, col_idx]).unwrap());
            }
            for row in offset..(offset+screen.shape()[0]) {
                *screen.get_mut([row % screen.shape()[0], col_idx]).unwrap() = queue.pop_front().unwrap()
            }
        },
        Inst::RotateRow((row_idx,offset)) => {
            let mut queue = VecDeque::new();
            for col in 0..screen.shape()[1] {
                queue.push_back(*screen.get([row_idx, col]).unwrap());
            }
            for col in offset..(offset+screen.shape()[1]) {
                *screen.get_mut([row_idx, col % screen.shape()[1]]).unwrap() = queue.pop_front().unwrap()
            }
        },
    }
}


fn parse_input(input: &str) -> Vec<Inst> {
    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_rot_col = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    let re_rot_row = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let mut instrs = Vec::new();

    for line in input.lines() {
        if let Some(cap) = re_rect.captures(line) {
            let x = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            instrs.push(Inst::Rect((x,y)));
        } else if let Some(cap) = re_rot_col.captures(line) {
            let x = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            instrs.push(Inst::RotateCol((x,y)));
        } else if let Some(cap) = re_rot_row.captures(line) {
            let x = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            instrs.push(Inst::RotateRow((x,y)));
        } else {
            unreachable!()
        }
    }
    instrs
}

#[allow(dead_code)]
fn format_screen(screen: &Array2<bool>) -> String {
    let mut output = String::new();
    for row in screen.rows() {
        if !output.is_empty() { output.push('\n') }
        for val in row.iter() {
            if *val { output.push('#') } else { output.push('.')}
        }
    }
    output
}


#[derive(Debug)]
enum Inst {
    Rect((usize,usize)),
    RotateCol((usize,usize)),
    RotateRow((usize,usize)),
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "rect 3x2\n\
    rotate column x=1 by 1\n\
    rotate row y=0 by 4\n\
    rotate column x=1 by 1";

    const TRUTH: &str =
    ".#..#.#\n\
    #.#....\n\
    .#.....";

    #[test]
    fn test1() {
        let instrs = parse_input(INPUT);
        let mut screen = Array2::default([3,7]);

        for inst in instrs {
            step(inst, &mut screen);
            println!("{}",format_screen(&screen));
            println!()
        }

        assert_eq!(TRUTH,format_screen(&screen));
    }

}