use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    p1(input);
}


fn p1(input: &str) {
    let code = parse_input(input);

    let mut input = 0;
    loop {
        let mut regs = HashMap::new();
        regs.insert('a', input);

        let output = run(&code, &mut regs, 10);
        println!("{}: {:?}", input, output);

        if output.into_iter().enumerate().all(|(i, v)| {
            if i % 2 == 0 {
                v == 0
            } else {
                v == 1
            }
        }) {
            break
        }

        input += 1;
    }
}


fn run(code: &[Op], regs: &mut HashMap<char, i64>, output_len: usize) -> Vec<i64> {
    let mut output = Vec::new();
    let mut pc = 0;

    while let Some(op) = code.get(pc) {
        match *op {
            Op::CPYimm(a, b) => {*regs.entry(b).or_default() = a; pc += 1},
            Op::CPYd(a, b) => {*regs.entry(b).or_default() = *regs.entry(a).or_default(); pc += 1},
            Op::INC(a) => {*regs.entry(a).or_default() += 1; pc += 1},
            Op::DEC(a) => {*regs.entry(a).or_default() -= 1; pc += 1},
            Op::JNZimm(a, b) => {if a != 0 { pc = pc.wrapping_add(b as usize) } else {pc += 1}},
            Op::JNZd(a, b) => {if *regs.entry(a).or_default() != 0 { pc = pc.wrapping_add(b as usize) } else {pc += 1}},
            Op::OUTd(a) => {
                output.push(*regs.entry(a).or_default());
                pc += 1;
            },
            Op::OUTi(a) => {
                output.push(a);
                pc += 1;
            }
        }
        if output.len() >= output_len {
            break
        }
    }

    output
}


fn parse_input(input: &str) -> Vec<Op> {
    let re = Regex::new(r"(?P<op>\w+) (?P<a>-?[a-z0-9]+)(?: )?(?P<b>-?[a-z0-9]+)?").unwrap();
    let mut output = Vec::new();
    for line in input.lines() {
        let cap = re.captures(line).unwrap_or_else(|| panic!("Unable to parse: \"{}\"",line));
        
        let op_str = cap.name("op").unwrap().as_str();
        let a = cap.name("a").unwrap().as_str();
        let b_opt = cap.name("b");

        let op = match op_str {
            "cpy" => {
                let b = b_opt.unwrap().as_str();
                if let Ok(val) = a.parse::<i64>() {
                    Op::CPYimm(val, b.chars().next().unwrap())
                } else {
                    Op::CPYd(a.chars().next().unwrap(), b.chars().next().unwrap())
                }
            },
            "inc" => {
                Op::INC(a.chars().next().unwrap())
            },
            "dec" => {
                Op::DEC(a.chars().next().unwrap())
            },
            "jnz" => {
                let b = b_opt.unwrap().as_str().parse::<i64>().unwrap();
                if let Ok(val) = a.parse::<i64>() {
                    Op::JNZimm(val, b)
                } else {
                    Op::JNZd(a.chars().next().unwrap(), b)
                }
            },
            "out" => {
                if let Ok(val) = a.parse::<i64>() {
                    Op::OUTi(val)
                } else {
                    Op::OUTd(a.chars().next().unwrap())
                }
            }
            _ => unreachable!("Invalid op {}",op_str),
        };
        output.push(op);
    }
    output
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
enum Op {
    CPYimm(i64,char),
    CPYd(char,char),
    INC(char),
    DEC(char),
    JNZimm(i64,i64),
    JNZd(char,i64),
    OUTd(char),
    OUTi(i64),
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "cpy 41 a\n\
    inc a\n\
    inc a\n\
    dec a\n\
    jnz a 2\n\
    dec a";


    #[test]
    fn test1() {
        let code = parse_input(INPUT);

        let mut regs = HashMap::new();
        run(&code, &mut regs);

        assert_eq!(regs[&'a'], 42);
    }
}