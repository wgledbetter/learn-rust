pub fn run() {
    // Default
    let v1 = "Fake";
    println!("v1 = {}", v1);

    // Mutable
    let mut v2 = 0;
    v2 = 5;
    println!("v2 = {}", v2);

    // Const
    const V3: i128 = 345;
    println!("V3 = {}", V3);

    // Multiple Variables
    let (t1, t2) = ("First", 45.6);
}
