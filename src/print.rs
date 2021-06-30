pub fn run() {
    // Print to console
    println!("Wow Fake");

    // Placeholder
    println!("{} is equal to {}", "wowo", 12);

    // Ordered
    println!("{0} is not {1} but is {2} by {1}", 12, 4, "divisible");

    // Named
    println!("{first} hard {second}", first = 2.34, second = "wow");

    // Formatted
    println!("Binary: {:b}\nHex: {:x}\nOctal: {:o}", 10, 10, 10);

    // Debug
    println!("{:?}", (12, true, "Wow"));

    // Expression
    println!("Expr = {}", 1 + 9);
}
