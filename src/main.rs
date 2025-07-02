#![allow(unused)]
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
    NotAdd
}

impl std::fmt::Display for AdditonError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"match error {:#?}",self)
    }
}

#[derive(Debug)]
enum SubstrictionError {
    NotSubstriction
}

impl std::fmt::Display for SubstrictionError {
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"match error {:#?}",self)
    }
}
impl std::error::Error for AdditonError {}
impl std::error::Error for SubstrictionError {}
fn addition(a:i32,b:i32) ->Result<i32,AdditonError> {
   Ok(a+b)
}
fn substriction(a:i32,b:i32) ->Result<i32,SubstrictionError> {
    Err(SubstrictionError::NotSubstriction)
 }
 fn total () ->Result<i32,Box<dyn std::error::Error>> {
    // ? operator handle error automatically 
    let x = addition(3,4)?;
    let y  = substriction(4,3)?;
    Ok(x * y )
    
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
   let assign_match_value =  match x {
       i @  1 ..3 => i,
        _ => 3
    };
    println!("{:#?}", assign_match_value );
    // Some 
    let x  = Some(3);
    match x {
        Some(x) => println!("{x} Some"),
        None =>
         println!("None")
    }
        let x : Result <i32,String>  = Ok(3);
        match x {
            Ok(x) => println!("{x}"),
            Err(msg) => println!("{msg}")
        }

    // if let (it works like this match x {
      //  Some(x) => println!("{x} Some"),
      //  None =>
       //  println!("None")
//    })
let v  = Some(3) ;
    if let Some(x) = v {
        println!("{x} is some ");
    }
    let  Some(x) = v else  {
        // here code must be return or diverge (panic)
     return 
    };

// Function 
let a = 3 ;
let b = 4; 
    let x   = total();
    match x {
        Ok(y) => println!("Add {a} {b} = {y}"),
        Err(error) => println!("{error}")
    }
    
 }

