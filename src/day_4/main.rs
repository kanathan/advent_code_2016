use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("{}",get_valid_id_sum(&parse(input)));
}


fn parse(input: &str) -> Vec<(String, u64, String)> {
    let mut output = Vec::new();
    let re = Regex::new(r"^([\w-]+)-(\d+)\[(\w+)\]$").unwrap();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let id = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let cksum = caps.get(3).unwrap().as_str().to_string();
        output.push((name,id,cksum));
    }


    output
}

fn get_valid_id_sum(items: &[(String, u64, String)]) -> u64 {
    let mut sum = 0;

    for (name, id, cksum) in items.iter() {
        let mut char_counts = HashMap::new();
        for c in name.chars() {
            if c == '-' { continue }
            *char_counts.entry(c).or_insert(0) += 1;
        }
        let sorted_chars = char_counts.into_iter().sorted_by(|a, b| {
            if a.1 == b.1 { a.0.cmp(&b.0) } else { b.1.cmp(&a.1) }
            //b.1.cmp(&a.1)
        });

        if sorted_chars.zip(cksum.chars()).all(|(a, b)| a.0 == b) {
            sum += id;
            println!("{}: {}", id, decrypt_name(name, *id));
        }
    }
    sum
}

fn decrypt_name(name: &str, id: u64) -> String {
    let mut output = String::new();
    let offset = (id % 26) as u8;

    for c in name.chars() {
        if c == '-' { output.push(' ') } else {
            output.push((((c as u8 - 97 + offset) % 26) + 97) as char)
        }
    }

    output
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "aaaaa-bbb-z-y-x-123[abxyz]\n\
    a-b-c-d-e-f-g-h-987[abcde]\n\
    not-a-real-room-404[oarel]\n\
    totally-real-room-200[decoy]";


    #[test]
    fn test1() {
        assert_eq!(get_valid_id_sum(&parse(INPUT)), 1514);
    }

    #[test]
    fn test2() {
        assert_eq!(decrypt_name("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
    }

}