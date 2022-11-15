use std::ops::RangeInclusive;



fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}


fn parse(input: &str) -> Vec<RangeInclusive<u32>> {
    let mut output = Vec::new();

    for line in input.lines() {
        let (start_str, stop_str) = line.split_once('-').unwrap();
        let start: u32 = start_str.parse().unwrap();
        let stop: u32 = stop_str.parse().unwrap();

        let range = start..=stop;
        output.push(range);
    }

    output
}


fn p1(input: &str) -> u32 {
    let ranges = parse(input);

    let mut val = 0;
    'outer: while val < u32::MAX {
        for range in ranges.iter() {
            if range.contains(&val) {
                val = range.end() + 1;
                continue 'outer
            }
        }
        return val
    }


    return val
}


fn p2(input: &str) -> u32 {
    let mut count = 0;
    let ranges = parse(input);

    let mut val = 0;
    'outer: while val <= u32::MAX {
        for range in ranges.iter() {
            if range.contains(&val) {
                if *range.end() != u32::MAX {
                    val = range.end() + 1;
                    continue 'outer
                } else {
                    break 'outer
                }
            }
        }
        count += 1;
        if val != u32::MAX { val += 1};
    }


    count
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test1() {
        
    }

}