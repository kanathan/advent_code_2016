use std::time;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    println!("{}",get_password("ojvtpuvg", 8));
    println!("{}",get_password_part2("ojvtpuvg", 8));
}


fn get_password(id: &str, length: usize) -> String {
    let time_now = time::Instant::now();
    let thread_count = 6;
    let cur_val = Arc::new(Mutex::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (tx,rx) = mpsc::channel();
    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let tx = tx.clone();
        let cur_val = cur_val.clone();
        let stop_flag = stop_flag.clone();
        let id = id.to_string();

        handles.push(thread::spawn(move || {
            loop {
                {
                    if stop_flag.load(Ordering::Relaxed) { break }
                }
                let key_val; 
                {
                    let mut cur_val = cur_val.lock().unwrap();
                    key_val = *cur_val;
                    *cur_val += 1;
                }
                let full_key = format!("{}{}",id,key_val);
                if let Some(hash) = valid_hash(&full_key) {
                    let send_result = tx.send(Some((key_val, hash)));
                    if send_result.is_err() { break }
                }
            }
            tx.send(None).unwrap();
        }));

    }

    let mut return_vals = Vec::new();

    loop {
        if let Ok(Some(return_val)) = rx.recv() {
            println!("Found {:?}",return_val);
            return_vals.push(return_val);
        }
        if return_vals.len() >= length {
            stop_flag.store(true, Ordering::Relaxed);
            break
        }
    }

    println!("Found enough results. Waiting for threads to complete");

    for _ in 0..thread_count {
        while let Ok(Some(return_val)) = rx.recv() {
            return_vals.push(return_val)
        }
    }

    println!("Found {} results", return_vals.len());
    println!("Completed in {} secs", time_now.elapsed().as_secs_f32());

    let sorted_return_vals = return_vals
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect_vec();

    sorted_return_vals[0..length].into_iter()
        .map(|(_, hash)| hash.chars().nth(5).unwrap())
        .collect()
}

fn get_password_part2(id: &str, length: usize) -> String {
    let time_now = time::Instant::now();
    let thread_count = 6;
    let cur_val = Arc::new(Mutex::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (tx,rx) = mpsc::channel();
    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let tx = tx.clone();
        let cur_val = cur_val.clone();
        let stop_flag = stop_flag.clone();
        let id = id.to_string();

        handles.push(thread::spawn(move || {
            loop {
                {
                    if stop_flag.load(Ordering::Relaxed) { break }
                }
                let key_val; 
                {
                    let mut cur_val = cur_val.lock().unwrap();
                    key_val = *cur_val;
                    *cur_val += 1;
                }
                let full_key = format!("{}{}",id,key_val);
                if let Some(hash) = valid_hash(&full_key) {
                    let send_result = tx.send(Some((key_val, hash)));
                    if send_result.is_err() { break }
                }
            }
            tx.send(None).unwrap();
        }));

    }

    let mut return_vals: HashMap<usize, (u64, char)>  = HashMap::new();

    loop {
        if let Ok(Some((key_val, hash))) = rx.recv() {
            println!("Found {}: {}",key_val, hash);
            if let Ok(val) = hash[5..6].parse::<usize>() {
                if val < length {
                    if !return_vals.contains_key(&val) || return_vals.get(&val).unwrap().0 > key_val {
                        return_vals.insert(val, (key_val, hash.chars().nth(6).unwrap()));
                    }
                }
            }
        }
        if return_vals.len() >= length {
            stop_flag.store(true, Ordering::Relaxed);
            break
        }
    }

    println!("Found enough results. Waiting for threads to complete");

    for _ in 0..thread_count {
        while let Ok(Some((key_val, hash))) = rx.recv() {
            println!("Found {}: {}",key_val, hash);
            if let Ok(val) = hash[5..6].parse::<usize>() {
                if val < length {
                    if !return_vals.contains_key(&val) || return_vals.get(&val).unwrap().0 > key_val {
                        return_vals.insert(val, (key_val, hash.chars().nth(6).unwrap()));
                    }
                }
            }
        }
    }

    println!("Completed in {} secs", time_now.elapsed().as_secs_f32());

    let mut code = (0..length).map(|_| '_').collect_vec();
    for (pos, (_, char)) in return_vals.into_iter() {
        *code.get_mut(pos).unwrap() = char;
    }
    
    code.into_iter().collect()
}


fn valid_hash(input: &str) -> Option<String> {
    let hash = format!("{:?}",md5::compute(input));
    if &hash[..5] == "00000" {
        Some(hash)
    } else {
        None
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_password("abc", 8), "18f47a30")
    }

}