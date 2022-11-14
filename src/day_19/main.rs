use std::collections::VecDeque;

fn main() {
    let elf = p1(3017957);
    println!("Last elf: {elf}");

    let elf = p2(3017957);
    println!("Last elf: {elf}");
}


fn p1(count: usize) -> usize {
    let mut elfs: VecDeque<usize> = (1..=count).collect();

    while elfs.len() > 1 {
        let front = elfs.pop_front().unwrap();
        elfs.push_back(front);
        elfs.pop_front();
    }

    return elfs[0]
}


fn p2(count: usize) -> usize {
    let mut r_elfs: VecDeque<usize> = (1..=(count/2)).collect();
    let mut l_elfs: VecDeque<usize> = ((count/2+1)..=(count)).collect();

    while r_elfs.len() > 0 {
        // Remove elf
        l_elfs.pop_front();

        // Move "cursor"
        l_elfs.push_back(r_elfs.pop_front().unwrap());

        // Rebalance if needed
        if l_elfs.len() > r_elfs.len() + 1 {
            r_elfs.push_back(l_elfs.pop_front().unwrap());
        }
    }


    l_elfs.pop_front().unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, p2(5));
    }

}