use core::f64;

pub fn run() {
    let arr: [f64; 4] = [2.0, 8.9, 6.4, 4.0];

    println!("{:?}", arr);

    println!("{}", arr[1]);

    println!("{} bytes", std::mem::size_of_val(&arr));

    // Slicing
    let s1: &[f64] = &arr[1..];
    println!("{:?}", s1);
}
