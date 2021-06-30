pub fn run() {
    let mut s1 = String::from("mulani");
    println!("{} is {} characters long", s1, s1.len());

    s1.push_str(", sameer");
    println!("{} is {} characters long", s1, s1.len());

    println!("Capacity: {}", s1.capacity());

    println!("Is Empty: {}", s1.is_empty());

    let s2 = String::from("meer");
    println!("{} contains {}? {}", s1, s2, s1.contains(&s2));

    for word in s1.split(", ") {
        println!("  {}", word);
    }

    let mut s3 = String::with_capacity(12);
    s3.push('a');
    s3.push('o');
    s3.push('e');
    s3.push('u');
    s3.push('i');
    s3.push('d');
    s3.push('h');
    s3.push('t');
    s3.push('n');
    s3.push('s');
}
