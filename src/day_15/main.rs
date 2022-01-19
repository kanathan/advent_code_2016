use regex::Regex;

fn main() {
    let input = include_str!("input");

    let mut discs = parse_input(input);

    println!("{}",find_first_time(discs.clone()));
    discs.push(Disc{pos:0,size:11});
    println!("{}",find_first_time(discs));

    
}

fn find_first_time(discs: Vec<Disc>) -> usize {
    let mut time = 0;
    loop {
        let mut step_time = time;
        if discs.iter().all(|d| {
            step_time += 1;
            (d.pos + step_time) % d.size == 0
        }) {
            return time
        }
        time += 1;
    }
}

fn parse_input(input: &str) -> Vec<Disc> {
    let re = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();

    let mut discs = Vec::new();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let size = cap.get(1).unwrap().as_str().parse().unwrap();
        let pos = cap.get(2).unwrap().as_str().parse().unwrap();
        discs.push(Disc{size, pos});
    }

    discs
}

#[derive(Clone)]
struct Disc {
    size: usize,
    pos: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "Disc #1 has 5 positions; at time=0, it is at position 4.\n\
    Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn test1() {
        assert_eq!(find_first_time(parse_input(INPUT)), 5);
    }

}