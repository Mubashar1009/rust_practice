#![allow(unused)]
#[derive(Debug)]
struct Language {
    name: String,
    number: i32,
}
fn main() {
    let language = "rust";
    let mut x = 1;
    // Integers
    // Signed Integers
    // integers : -(2**(n-1)) ~ 2**(n-1) -1
    // Unsigned Integers : 0 ~ 2**(n) -1
    // isize is the datatype which depend upon the architecture of the computer
    let i: i8 = 3; // -(2**(8-1)) ~ 2(8-1) - 1 = -128 ~ 127
    // type converstion
    let y = i;
    let min_value: i8 = i8::MIN;
    let max_value: i8 = i8::MAX;
    let proper_sentense = Language {
        name: "Rust".to_string(),
        number: 1,
    };
    let vect: Vec<_> = vec![23, 23, -34];
    println!("Hello {language}");
    println!("Number {0} is {1}", x, language);
    println!("{:#?}", proper_sentense);
    println!("Min value of i8 {min_value} and Max value of i8 {max_value} {y} {i}");
    println!("checked_add :{:?}", i8::checked_add(i8::MAX, 2));
}
