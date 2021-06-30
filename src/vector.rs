pub fn run() {
    let mut vec: Vec<f64> = vec![];

    vec.push(3.4);
    vec.push(1.98);

    for x in vec.iter() {
        println!("{}", x);
    }

    for x in vec.iter_mut() {
        *x *= 3.0;
    }

    for x in vec.iter() {
        println!("{}", x);
    }
}
