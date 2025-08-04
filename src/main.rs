#![allow(unused)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

#[derive(Debug, PartialEq)]

// enum Color {
//     Red,
//     Green,
//     RGBA(u32,u32,u32,f32),
//     HSL{
//         h:u32,
//         s:u32,
//         l:u32
//     }
// }

struct Language {
    name: String,
    number: i32,
}

impl Language {
    fn assign_value() -> Self {
        Self {
            name: "Mubashir".to_string(),
            number: 12,
        }
    }

    fn mut_value(&mut self, x: String, y: i32) {
        self.name = x;
        self.number = y;
    }
}

#[derive(Debug)]
enum AdditonError {
    NotAdd,
}
use rust_project::module1;

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
    // let proper_sentense = Language {
    //     name: "Rust".to_string(),
    //     number: 1,
    // };
    let vect: Vec<_> = vec![23, 23, -34];
    println!("Hello {language}");
    println!("Number {0} is {1}", x, language);
    // println!("{:#?}", proper_sentense);
    println!("Min value of i8 {min_value} and Max value of i8 {max_value} {y} {i}");
    println!("checked_add :{:?}", i8::checked_add(i8::MAX, 2));

    //tuple
    let tuple: (bool, i32, char) = (false, 3, 'm');
    // Nested Tuple
    let nested_tuple = ((3, 2, 'c'), 9, ("Mubasher".to_string()));
    println!("nestedtuple = {}", nested_tuple.2);

    // Array
    let arr = [3, 4, 2, 3];
    //slices
    let slice = &arr[..3];
    println!("slice {:?}", slice);

    // String
    let mut name = String::from("Mubashir");
    name += " Shakeel";
    println!("My name:{name}");
    //&str
    let s = r#"Mubashir Shakeel
   BSIT"#;
    let p = "developer";
    let msg = format!("{} {}", s, p);
    println!("{msg}");

    // Enum
    //    let coloring = Color::Red;
    //    let coloring  = Color::HSL{
    //     h:1,
    //     s:3,
    //     l:1
    //    };
    // struct
    let mut p = Language::assign_value();
    p.mut_value("Mubasher".to_string(), 34);
    println!("{:?}", p);
    // operators
    let first_number = 3;
    let second_number = 4;
    let add = first_number + second_number;
    let sub = second_number - first_number;
    let boolean = true && false;
    let reminder = second_number % first_number;
    println!("{boolean} reminder {second_number} % {first_number}= {reminder}");
    // bitwise
    println!("{:03b}", 4 >> 1);
    // if else
    let x = 4;
    let y = if x > 4 {
        5
    } else if x == 4 {
        6
    } else {
        1
    };
    println!("{y} is");
    // loops
    let mut i = 1;
    loop {
        if i == 3 {
            break;
        }
        i += 1;
        println!("hi {i}")
    }
    while i == 3 {
        if i == 3 {
            break;
        }
        println!("{i} while loop")
    }
    for i in 0..=3 {
        println!("{i} in loop")
    }
    //  vectors and iter
    // iter is immutable refrence and it is only read only withoug using this ownership transfer
    let vec = vec![2, 3, 1];
    for i in vec.iter() {
        println!("{i} vector with iter");
    }
    // label
    'outer: for i in 0..3 {
        'iner: for j in 0..2 {
            if i == 2 && j == 2 {
                break 'outer;
            }
        }
    }
    // Match
    let x = 4;
    let assign_match_value = match x {
        i @ 1..3 => i,
        _ => 3,
    };
    println!("{:#?}", assign_match_value);
    // Some
    let x = Some(3);
    match x {
        Some(x) => println!("{x} Some"),
        None => println!("None"),
    }
    let x: Result<i32, String> = Ok(3);
    match x {
        Ok(x) => println!("{x}"),
        Err(msg) => println!("{msg}"),
    }

    // if let (it works like this match x {
    //  Some(x) => println!("{x} Some"),
    //  None =>
    //  println!("None")
    //    })
    let v = Some(3);
    if let Some(x) = v {
        println!("{x} is some ");
    }
    let Some(x) = v else {
        // here code must be return or diverge (panic)
        return;
    };

    // Function
    let a = 3;
    let b = 4;
    // let x   = addition (a,b);
    println!("Add {a} {b} = {x}");
    module1::first_function();
    let x = [3, 4, 5];
    // Error Handling
    let get_element = x.get(1);
    match get_element {
        Some(value) => println!("{value} is present"),
        None => println!("there is no value at this index"),
    }
    // alternative method
    let result = get_element.expect("there is no element");
    println!("{result} is ");
    // slice are the refrence of the collection
    let arr = [1, 2, 3, 4, 5];
    fn slice_arr(u: &[i32], i: usize) -> (&[i32], &[i32]) {
        (&u[0..i], &u[i..])
    }
    let (a, b) = slice_arr(&arr, 2);
    println!("{:#?}", a);
    //Dereferencing is accessing the value that a reference points to. This is done using *
    let x = 3;
    let y = &x;
    let z = &x;
    let yz = *y + *z;
    println!("{yz} {x}");
    // vector are those which size can be shrink or extend on compiler time
    let v = vec![1u8; 5]; // here by using first item with type vector can understand that all items will be of u8;
    let mut v = Vec::new(); // it create empty vector 

    let x = v.get(0); // it give options 
    // for adding and updating we need to convert vector into mutable
    v.push(3);
    v[0] = 2; // update vector value 
    println!("{:#?}", v);
    //Hashmap key value pair
    let mut score = HashMap::new();
    score.insert("red".to_string(), 200);
    score.insert("green".to_string(), 210);
    let get_score = score.get("red"); // it work on refrence 
    let v = score.entry("blue".to_string()).or_insert(300);
    *v = 55;
    // HashSet insert unique values and give boolean value if present or not a give number
    let mut unique = HashSet::new();
    let boolean = unique.insert(3);
    println!("{boolean}");
    let boolean = unique.insert(3);
    println!("{boolean}");
}
