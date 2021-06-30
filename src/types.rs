pub fn run() {
    let v1 = 1;

    let v2 = 3.14;

    let v3: i128 = 2345098712349876134509876432560876;

    let v4 = std::f64::MAX;

    let v5 = std::i16::MIN;

    let v6: bool = "true".contains("ru");

    println!("{:?}", (v1, v2, v3, v4, v5, v6));
}
