pub fn run() {
    // Conditionals
    let wow = 123;
    let fake = 4.12;

    if wow >= 666 {
        println!("Good");
    } else if fake % 3.2 > 2.0 && wow > 333 {
        println!("Bad");
    }

    let dumb = if wow < 321 { "aoeu" } else { "snth" };
}
