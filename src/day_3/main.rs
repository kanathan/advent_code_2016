use itertools::{Itertools,multizip};



fn main() {
    let input = include_str!("input");
    println!("{}",get_valid_triangles(input));
    println!("{}",get_valid_triangles_vert(input));
}

fn get_valid_triangles(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        let mut sorted_vals = line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        sorted_vals.sort();

        if sorted_vals[0] + sorted_vals[1] > sorted_vals[2] { count += 1 }
    }

    count
}

fn get_valid_triangles_vert(input: &str) -> u64 {
    let mut count = 0;

    for (line1, line2, line3) in input
            .lines()
            .map(|l| l.split_whitespace().map(|s| s.parse::<u64>().unwrap()))
            .tuples() {
        
        for vals in multizip((line1, line2, line3)).map(|(x,y,z)| { let mut v = vec![x,y,z]; v.sort(); v }) {
            if vals[0] + vals[1] > vals[2] { count += 1 }
        }
    }

    count
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "101 301 501\n\
    102 302 502\n\
    103 303 503\n\
    201 401 601\n\
    202 402 602\n\
    203 403 603";

    #[test]
    fn test1() {
        get_valid_triangles_vert(INPUT);
    }

}