use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = include_str!("input");
    println!("{}", get_tot_len(input));
    println!("{}", get_tot_len_p2(input));
}

fn get_tot_len(input: &str) -> usize {
    let mut len = 0;

    for line in input.lines() {
        len += parse_line(line).len();
    }

    len
}

fn get_tot_len_p2(input: &str) -> usize {
    let mut len = 0;

    for line in input.lines() {
        len += calc_line_length_p2(line);
    }

    len
}

fn parse_line(line: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    }

    let mut output = String::new();
    let mut cur = 0;
    loop {
        if let Some(cap) = RE.captures(&line[cur..]) {
            let start = cap.get(0).unwrap().start() + cur;
            let end = cap.get(0).unwrap().end() + cur;
            let ch_len = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let repeat = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
            output.push_str(&line[cur..start]);
            for _ in 0..repeat {
                output.push_str(&line[end..(end+ch_len)]);
            }
            cur = end + ch_len;
        } else {
            output.push_str(&line[cur..]);
            break
        }
    }

    output
}

fn calc_line_length_p2(line: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    }

    let mut size = 0;

    if let Some(cap) = RE.captures(line) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();
        let ch_len = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let repeat = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        size += start;
        size += repeat * calc_line_length_p2(&line[end..(end+ch_len)]);
        size += calc_line_length_p2(&line[(end+ch_len)..])
    } else {
        size += line.len();
    }

    size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(parse_line("ADVENT"), "ADVENT");
        assert_eq!(parse_line("A(1x5)BC"), "ABBBBBC");
        assert_eq!(parse_line("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(parse_line("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(parse_line("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(parse_line("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test2() {
        assert_eq!(calc_line_length_p2("(3x3)XYZ"),"XYZXYZXYZ".len());
        assert_eq!(calc_line_length_p2("X(8x2)(3x3)ABCY"),"XABCABCABCABCABCABCY".len());
        assert_eq!(calc_line_length_p2("(27x12)(20x12)(13x14)(7x10)(1x12)A"),241920);
        assert_eq!(calc_line_length_p2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),445);
    }

}