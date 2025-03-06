use std::collections::HashSet;

const OPERATIONS: [usize; 7] = [100, 500, 1000, 5000, 10000, 50000, 100000];

fn harmonic(n: usize) -> f64 {
    (1..=n).map(|x| 1.0 / x as f64).sum()
}

struct Node {
    data: usize,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
}

fn main() {
    let operation_stamps = HashSet::from(OPERATIONS);
    let h_100 = harmonic(100);
    println!("Harmonic 100: {}", h_100);
    let mut linked_list = LinkedList::new();
}
