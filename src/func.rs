pub fn run() {
    print_sum(3, 9);

    let mySum = add_int(3, 6);

    // Closure
    let added = |n1: i32, n2: i32| n1 + n2;
    println!("Lambda equivalent? {}", added(3, 1));
}

fn print_sum(lhs: i32, rhs: i32) {
    println!("{}", lhs + rhs);
}

fn add_int(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}
