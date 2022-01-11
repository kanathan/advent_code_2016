use fancy_regex::Regex;

fn main() {
    let input = include_str!("input");
    println!("{}",get_tls_count(input));
    println!("{}",get_ssl_count(input));
}


fn get_tls_count(input: &str) -> u64 {
    let re_line = Regex::new(r"(\w+)(\[\w+\])?").unwrap();
    let re_pattern = Regex::new(r"(\w)((?!\1)\w)\2\1").unwrap();

    let mut count = 0;

    for line in input.lines() {
        let mut unbracketed = Vec::new();
        let mut bracketed = Vec::new();
        for capture in re_line.captures_iter(line) {
            if let Ok(capture) = capture {
                unbracketed.push(capture.get(1).unwrap().as_str());
                if let Some(item) = capture.get(2) {
                    bracketed.push(item.as_str());
                }
            }
        }
        if unbracketed.into_iter().any(|s| re_pattern.is_match(s).unwrap())
        && bracketed.into_iter().all(|s| !re_pattern.is_match(s).unwrap()) {
            count += 1;
        }
    }


    count
}

fn get_ssl_count(input: &str) -> u64 {
    let re_line = Regex::new(r"(\w+)(\[\w+\])?").unwrap();
    let re_pattern = Regex::new(r"(\w)((?!\1)\w)\1").unwrap();

    let mut count = 0;

    'line_loop: for line in input.lines() {
        let mut unbracketed = Vec::new();
        let mut bracketed = Vec::new();
        for capture in re_line.captures_iter(line) {
            if let Ok(capture) = capture {
                unbracketed.push(capture.get(1).unwrap().as_str());
                if let Some(item) = capture.get(2) {
                    bracketed.push(item.as_str());
                }
            }
        }
        for ubs in unbracketed {
            let mut pos = 0;
            while pos < ubs.len() {
                if let Ok(Some(cap)) = re_pattern.captures(&ubs[pos..]) {
                    let c1 = cap.get(1).unwrap().as_str();
                    let c2 = cap.get(2).unwrap().as_str();
                    let s = format!("{}{}{}",c2,c1,c2);
                    //println!("With {} in {} looking for {} in bracketed",cap.get(0).unwrap().as_str(),&ubs[pos..],s);
                    for bs in bracketed.iter() {
                        if bs.contains(&s) {
                            //println!("\tFound match!");
                            count += 1;
                            continue 'line_loop
                        }
                    }
                    pos += cap.get(0).unwrap().start() + 1;
                } else {
                    break
                }
            }
        }
    }


    count
}



#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "abba[mnop]qrst\n\
    abcd[bddb]xyyx\n\
    aaaa[qwer]tyui\n\
    ioxxoj[asdfgh]zxcvbn";

    const INPUT2: &str =
    "aba[bab]xyz\n\
    xyx[xyx]xyx\n\
    aaa[kek]eke\n\
    zazbz[bzb]cdb";

    #[test]
    fn test1() {
        //dbg!(get_tls_count(INPUT));
        dbg!(get_ssl_count(INPUT2));
    }

}