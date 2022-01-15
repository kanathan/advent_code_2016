use std::time;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use fancy_regex::Regex;

fn main() {
    println!("{}",get_pad_idx(64, "qzyelonm", false));
    println!("{}",get_pad_idx(64, "qzyelonm", true));
}


fn get_pad_idx(nth: usize, salt: &'static str, hash_stretch: bool) -> usize {
    let cur_time = time::Instant::now();
    let mut hasher = Hasher::new(salt, hash_stretch);

    let mut pads = Vec::new();
    while pads.len() < nth {
        let info = hasher.next();
        if let Some(c) = info.triple {
            if hasher.get_fives(c).is_some() {
                pads.push(info.index);
            }
        }
    }
    println!("Found pad {} after {} secs",nth,cur_time.elapsed().as_secs_f32());
    pads.into_iter().last().unwrap()
}


struct Hasher {
    salt: &'static str,
    queue: VecDeque<HashInfo>,
    cur_idx: usize,
    hash_stretch: bool,
}

impl Hasher {
    fn new(salt: &'static str, hash_stretch: bool) -> Self {
        let queue = VecDeque::new();
        Self {
            salt,
            queue,
            cur_idx: 0,
            hash_stretch
        }
    }

    fn next_hash(&mut self) {
        let hash = self.get_hash();
        let triple = get_triple(&hash);
        let fives = get_fives(&hash);

        self.queue.push_back(HashInfo {
            index: self.cur_idx,
            triple,
            fives,
        });

        self.cur_idx += 1;
    }


    fn get_hash(&self) -> String {
        if self.hash_stretch {
            let mut temp_hash = format!("{:?}",md5::compute(format!("{}{}",self.salt,self.cur_idx)));
            for _ in 0..2016 {
                temp_hash = format!("{:?}",md5::compute(temp_hash));
            }
            temp_hash
        } else {
            format!("{:?}",md5::compute(format!("{}{}",self.salt,self.cur_idx)))
        }
    }

    fn next(&mut self) -> HashInfo {
        if self.queue.is_empty() {
            self.next_hash();
        }
        self.queue.pop_front().unwrap()
    }

    fn get_fives(&mut self, c: char) -> Option<usize> {
        while self.queue.len() < 1000 {
            self.next_hash();
        }
        for info in self.queue.iter().take(1000) {
            if info.fives.contains(&c) { return Some(info.index) }
        }
        None
    }
}


struct HashInfo {
    index: usize,
    triple: Option<char>,
    fives: Vec<char>,
}


fn get_triple(hash: &str) -> Option<char> {
    lazy_static! {
        static ref RE_TRIPLE: Regex = Regex::new(r"(\w)\1\1").unwrap();
    }

    if let Ok(Some(m)) = RE_TRIPLE.find(hash) {
        Some(hash.chars().nth(m.start()).unwrap())
    } else {
        None
    }
}

fn get_fives(hash: &str) -> Vec<char> {
    lazy_static! {
        static ref RE_5X: Regex = Regex::new(r"(\w)\1\1\1\1").unwrap();
    }

    let mut found_chars = Vec::new();
    for m in RE_5X.find_iter(hash) {
        found_chars.push(hash.chars().nth(m.unwrap().start()).unwrap())
    }

    found_chars
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_pad_idx(1, "abc", false), 39);
        assert_eq!(get_pad_idx(2, "abc", false), 92);
        assert_eq!(get_pad_idx(64, "abc", false), 22728);
    }

    #[test]
    fn test2() {
        let hasher = Hasher::new("abc", true);
        assert_eq!(hasher.get_hash(),"a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn test3() {
        assert_eq!(get_pad_idx(1, "abc", true), 10);
        assert_eq!(get_pad_idx(64, "abc", true), 22551);
    }

}