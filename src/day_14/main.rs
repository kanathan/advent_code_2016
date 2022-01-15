use std::time;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
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

type HashGen = (Arc<Mutex<VecDeque<HashInfo>>>, mpsc::Receiver<()>);

struct Hasher {
    salt: &'static str,
    queue: VecDeque<HashInfo>,
    cur_idx: usize,
    hash_stretch: bool,
    hash_gen: Option<HashGen>,
}

impl Hasher {
    fn new(salt: &'static str, hash_stretch: bool) -> Self {
        let queue = VecDeque::new();
        let hash_gen = generate_hashes(salt, 0, 2000, hash_stretch);

        Self {
            salt,
            queue,
            cur_idx: 2000,
            hash_stretch,
            hash_gen: Some(hash_gen),
        }
    }

    fn wait_until_hashes_available(&mut self, count: usize) {
        loop {
            // Make sure generator is running at least 1000 hashes, even if we already have enough
            if self.hash_gen.is_none() {
                
                self.hash_gen = Some(generate_hashes(self.salt, self.cur_idx, count.max(1000), self.hash_stretch));
                self.cur_idx += count.max(1000);
            }

            if self.queue.len() >= count {
                return
            }

            {   
                let mut new_queue = self.hash_gen.as_mut().unwrap().0.lock().unwrap();
                self.queue.append(&mut new_queue);
            }
            if self.hash_gen.as_ref().unwrap().1.try_recv().is_ok() {
                self.hash_gen = None;
            };
        }
    }

    fn next(&mut self) -> HashInfo {
        self.wait_until_hashes_available(1);
        self.queue.pop_front().unwrap()
    }

    fn get_fives(&mut self, c: char) -> Option<usize> {
        self.wait_until_hashes_available(1000);
        for info in self.queue.iter().take(1000) {
            if info.fives.contains(&c) { return Some(info.index) }
        }
        None
    }
}


fn generate_hashes(salt: &'static str, start_idx: usize, count: usize, hash_stretch: bool) -> HashGen {
    let output = Arc::new(Mutex::new(VecDeque::new()));
    let output_returned = output.clone();
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let thread_count = 6;
        let mut handles = Vec::new();

        for idx in (start_idx..(count+start_idx)).step_by(count/thread_count + 1) {
            let start = idx;
            let stop = (idx+(count/thread_count + 1)).min(start_idx+count);
            
            handles.push(thread::spawn(move || {
                let mut hashes = VecDeque::new();

                for idx in start..stop {
                    let hash =
                    if hash_stretch {
                        let mut temp_hash = format!("{:?}",md5::compute(format!("{}{}",salt,idx)));
                        for _ in 0..2016 {
                            temp_hash = format!("{:?}",md5::compute(temp_hash));
                        }
                        temp_hash
                    } else {
                        format!("{:?}",md5::compute(format!("{}{}",salt,idx)))
                    };
                    let triple = get_triple(&hash);
                    let fives = get_fives(&hash);
                    hashes.push_back(HashInfo {
                        index: idx,
                        triple,
                        fives,
                    });
                }

                hashes
            }));

        }

        for handle in handles {
            let mut hashes = handle.join().unwrap();
            {
                output.lock().unwrap().append(&mut hashes);
            }
        }

        tx.send(()).expect("Error sending message");
    });

    (output_returned, rx)
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
        assert_eq!(get_pad_idx(1, "abc", true), 10);
        assert_eq!(get_pad_idx(64, "abc", true), 22551);
    }

}