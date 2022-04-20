fn main() {
    println!("{}", fibonacci_sequence(50));
}

fn fibonacci_sequence(n: u8) -> u64 {
    let mut prev_num: u64 = 0;
    let mut curr_num: u64 = 1;

    for _ in 1..n {
        let next = prev_num + curr_num;
        prev_num = curr_num;
        curr_num = next;
    }

    return curr_num;
}
