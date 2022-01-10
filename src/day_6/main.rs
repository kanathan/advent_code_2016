use std::collections::HashMap;

fn main() {
    let input = include_str!("input");

    println!("{}",decode_p1(input));
    println!("{}",decode_p2(input));
}


fn decode_p1(input: &str) -> String {

    let char_count_map = sum_chars(input);

    char_count_map.into_iter()
        .map(|h| {
            h.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
        })
        .collect()
}

fn decode_p2(input: &str) -> String {

    let char_count_map = sum_chars(input);

    char_count_map.into_iter()
        .map(|h| {
            h.into_iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().0
        })
        .collect()
}


fn sum_chars(input: &str) -> Vec<HashMap<char, u64>> {
    let line_size = input.lines().next().unwrap().len();
    let mut char_count_map = vec![HashMap::new(); line_size];

    for line in input.lines() {
        for (pos, c) in line.chars().enumerate() {
            *char_count_map.get_mut(pos).unwrap().entry(c).or_insert(0) += 1;
        }
    }
    char_count_map
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "eedadn\n\
    drvtee\n\
    eandsr\n\
    raavrd\n\
    atevrs\n\
    tsrnev\n\
    sdttsa\n\
    rasrtv\n\
    nssdts\n\
    ntnada\n\
    svetve\n\
    tesnvt\n\
    vntsnd\n\
    vrdear\n\
    dvrsen\n\
    enarar";

    #[test]
    fn test1() {
        assert_eq!(decode_p1(INPUT), "easter");
    }

    #[test]
    fn test2() {
        assert_eq!(decode_p2(INPUT), "advent");
    }

}