mod array;
mod ass;
mod cli;
mod cond;
mod enums;
mod func;
mod loops;
mod print;
mod ptr_ref;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vector;

fn main() {
    print::run();
    println!("-----------------------------------------------");

    vars::run();
    println!("-----------------------------------------------");

    types::run();
    println!("-----------------------------------------------");

    strings::run();
    println!("-----------------------------------------------");

    // ass::run();
    // println!("-----------------------------------------------");

    tuples::run();
    println!("-----------------------------------------------");

    array::run();
    println!("-----------------------------------------------");

    vector::run();
    println!("-----------------------------------------------");

    cond::run();
    println!("-----------------------------------------------");

    loops::run();
    println!("-----------------------------------------------");

    func::run();
    println!("-----------------------------------------------");

    ptr_ref::run();
    println!("-----------------------------------------------");

    structs::run();
    println!("-----------------------------------------------");

    enums::run();
    println!("-----------------------------------------------");

    cli::run();
    println!("-----------------------------------------------");
}
