use itertools::Itertools;



fn main() {
    println!("{}",part1("10001110011110000", 272));
    println!("{}",part1("10001110011110000", 35651584));
}


fn generate_data(mut a: String, requested_length: usize) -> String {
    let b: String = a.chars().rev().map(|c| match c {
        '0' => '1',
        '1' => '0',
        _ => unreachable!(),
    }).collect();
    a.push('0');
    a.push_str(&b);
    if a.len() >= requested_length {
        a
    } else {
        generate_data(a, requested_length)
    }
}

fn get_checksum(input: &str) -> String {
    let mut checksum = input.to_string();
    loop {
        checksum = checksum
            .chars()
            .tuples::<(_,_)>()
            .map(|(a, b)| {
                if a==b { '1' } else { '0' }
            })
            .collect();
        if checksum.len() % 2 == 1 { break }
    }
    checksum
}

fn part1(input: &str, length: usize) -> String {
    let data = generate_data(input.to_string(), length);
    get_checksum(&data[..length])
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(generate_data("1".to_string(), 2), "100");
        assert_eq!(generate_data("0".to_string(), 2), "001");
        assert_eq!(generate_data("11111".to_string(), 6), "11111000000");
        assert_eq!(generate_data("111100001010".to_string(), 13), "1111000010100101011110000");
    }

    #[test]
    fn test2() {
        assert_eq!(get_checksum("110010110100"), "100");
    }

    #[test]
    fn test3() {
        assert_eq!(part1("10000", 20), "01100");
    }

}