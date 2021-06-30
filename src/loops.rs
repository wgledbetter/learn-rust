pub fn run() {
    //
    let mut i = 0;

    loop {
        i += 1;

        println!("{}", i);

        if i >= 20 {
            break;
        }
    }

    // While
    while i <= 100 {
        if i % 15 == 0 {
            println!("ThreeFive");
        } else if i % 5 == 0 {
            println!("Five");
        } else if i % 3 == 0 {
            println!("Three");
        } else {
            println!("{}", i);
        }

        i += 1;
    }

    println!("");
    println!("");

    // For
    for x in 0..75 {
        if x % 15 == 0 {
            println!("ThreeFive");
        } else if x % 5 == 0 {
            println!("Five");
        } else if x % 3 == 0 {
            println!("Three");
        } else {
            println!("{}", x);
        }
    }
}
