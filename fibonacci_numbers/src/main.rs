use std::io;

fn main() {
    let mut input = String::new();
    println!("nth fibonacci number: ");
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", fibonacci(input.trim().parse().unwrap()));
}

fn fibonacci(n: usize) -> usize {
    let mut next = 1;
    let mut curr = 0;
    for _ in 1..n {
        let new_next = curr + next;
        curr = next;
        next = new_next;
    }
    curr
}
