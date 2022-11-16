use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("P1: {}", p1(input));

    println!("P2: {}", p2(input));
}


fn p1(input: &str) -> i64 {
    let mut code = parse_input(input);
    let mut regs = HashMap::from([('a', 7)]);
    run(&mut code, &mut regs);
    regs[&'a']
}

fn p2(input: &str) -> i64 {
    let mut code = parse_input(input);
    let mut regs = HashMap::from([('a', 12)]);
    run(&mut code, &mut regs);
    regs[&'a']
}


fn run(code: &mut Vec<Op>, regs: &mut HashMap<char, i64>) {
    let mut opts = optimize(code);

    let mut pc = 0;

    while pc < code.len() {
        let (op, pc_inc) = 
            if opts.contains_key(&pc) {
                *opts.get(&pc).unwrap()
            } else {
                let op = *code.get(pc).unwrap();
                let pc_inc = 1;
                (op, pc_inc)
            };

        match op {
            Op::CPY(a, b) => {
                if let Arg::Reg(c) = b {
                    *regs.entry(c).or_default() = a.eval(regs);
                }
                pc += pc_inc;
            },
            Op::INC(a) => {
                if let Arg::Reg(c) = a {
                    *regs.entry(c).or_default() += 1;
                }
                pc += pc_inc;
            },
            Op::DEC(a) => {
                if let Arg::Reg(c) = a {
                    *regs.entry(c).or_default() -= 1;
                }
                pc += pc_inc;
            },
            Op::JNZ(a, b) => {
                if a.eval(regs) != 0 {
                    pc = pc.wrapping_add(b.eval(regs) as usize);
                } else {
                    pc += pc_inc;
                }
            },
            Op::TGL(a) => {
                let idx = pc.wrapping_add(a.eval(regs) as usize);
                toggle_instr(code, idx);

                opts = optimize(code);

                pc += pc_inc;
            },
            Op::MULADD(a, b, c) => {
                if let Arg::Reg(ch) = c {
                    let result = a.eval(regs) * b.eval(regs);
                    *regs.entry(ch).or_default() += result;
                }
                pc += pc_inc;
            },
        }
    }
}


fn optimize(code: &Vec<Op>) -> HashMap<usize,(Op, usize)> {
    let mut opts = HashMap::new();
    
    let mut pc = 0;
    while pc + 4 < code.len() {
        if let Op::INC(a) = code[pc] {
            if let Op::DEC(c) = code[pc+1] {
                if let Op::DEC(d) = code[pc+3] {
                    let line2 = code[pc+2];
                    let line4 = code[pc+4];
                    if matches!(Op::JNZ(c, Arg::Val(-2)), line2) &&
                       matches!(Op::JNZ(d, Arg::Val(-5)), line4)
                    {
                        opts.insert(pc, (Op::MULADD(c, d, a), 5));
                    }
                }
            }
        }
        pc += 1;
    }

    opts
}


fn toggle_instr(code: &mut Vec<Op>, pc: usize) {
    if let Some(instr) = code.get_mut(pc) {
        match *instr {
            Op::CPY(a, b) => *instr = Op::JNZ(a, b),
            Op::INC(a) => *instr = Op::DEC(a),
            Op::DEC(a) => *instr = Op::INC(a),
            Op::JNZ(a, b) => *instr = Op::CPY(a, b),
            Op::TGL(a) => *instr = Op::INC(a),
            _ => unreachable!("No toggle for {:?}", instr)
        }
    }
}

fn parse_input(input: &str) -> Vec<Op> {
    let re = Regex::new(r"(?P<op>\w+) (?P<a>-?[a-z0-9]+)(?: )?(?P<b>-?[a-z0-9]+)?").unwrap();
    let mut output = Vec::new();
    for line in input.lines() {
        let cap = re.captures(line).unwrap_or_else(|| panic!("Unable to parse: \"{}\"",line));
        
        let op_str = cap.name("op").unwrap().as_str();
        let a_str = cap.name("a").unwrap().as_str();
        let b_opt = cap.name("b");

        let a = if let Ok(val) = a_str.parse::<i64>() {
            Arg::Val(val)
        } else {
            Arg::Reg(a_str.chars().next().unwrap())
        };

        let b = b_opt.map(|m| {
            if let Ok(val) = m.as_str().parse::<i64>() {
                Arg::Val(val)
            } else {
                Arg::Reg(m.as_str().chars().next().unwrap())
            }
        });

        let op = match op_str {
            "cpy" => Op::CPY(a, b.unwrap()),
            "inc" => Op::INC(a),
            "dec" => Op::DEC(a),
            "jnz" => Op::JNZ(a, b.unwrap()),
            "tgl" => Op::TGL(a),
            _ => unreachable!("Invalid op {}",op_str),
        };
        output.push(op);
    }
    output
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
enum Op {
    CPY(Arg,Arg),
    INC(Arg),
    DEC(Arg),
    JNZ(Arg, Arg),
    TGL(Arg),
    MULADD(Arg, Arg, Arg),
}


#[derive(Clone, Copy, Debug)]
enum Arg {
    Reg(char),
    Val(i64),
}

impl Arg {
    fn eval(&self, regs: &mut HashMap<char, i64>) -> i64 {
        match self {
            Arg::Reg(a) => *regs.entry(*a).or_default(),
            Arg::Val(a) => *a,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "cpy 2 a\n\
    tgl a\n\
    tgl a\n\
    tgl a\n\
    cpy 1 a\n\
    dec a\n\
    dec a";

    #[test]
    fn test1() {
        let mut code = parse_input(INPUT);
        let mut regs = HashMap::new();
        run(&mut code, &mut regs);
        assert_eq!(regs[&'a'], 3);
    }

}