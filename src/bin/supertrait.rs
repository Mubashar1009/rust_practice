#![allow(unused)]
use std::ops::Add;
use std::convert::{From,Into};
trait Language {
    fn name(&self) -> String;
}

trait Compiler {
    fn compiler(&self,file_name:&str) -> String ;
}

trait CompilerLanguage: Language + Compiler {
    fn execute (&self, file_name: &str) {
        // here self will contain both trait functions name and compiler;
        println!("name {}",self.name());
        let file_name = self.compiler(file_name);
        println!("file_name : {file_name}");
    }
}
#[derive(Debug, PartialEq)]
// type must have both traits Compiler and Language then super trait will work 
struct Rust;
impl Language for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}

impl Compiler for Rust {
    fn compiler(&self,file_name:&str) -> String {
        format!("{file_name}")
    }

}

impl CompilerLanguage for Rust {
    // here we does not change execute function then it will work as defualt inilitation 
}

impl From<String> for Rust {
    fn from (value:String) -> Self {
        // Self refer to Rust 
        Self 
    }
}

// New function to demonstrate `into`
fn into_usage() {
    let a: Rust = "Mubashir".to_string().into(); 
    println!("Converted using into(): {:#?}", a);
}

// Generic data type 

enum Result<T,E> {
    Ok(T),
    Err(E)
}
// default Generic Type 
#[derive(Debug)]
struct Point<T  = i32> {
    p : T,
    s : T,
}
fn sum<T: std::ops::Add<E, Output = E>,E>(a:T,b:E)->E {
    a+b
}
// Generic Traits 

trait GenericTraits<T> {
    fn sum(&self)->&T;
}
impl<T> GenericTraits<T> for (T,T) {
  fn   sum(&self)->&T {
        &self.0
    }
}

// Trait bounds 
trait A {

}
trait B {} 


impl A for u32 {}
impl A for i32 {}
impl B for u32 {}
// These are the example of static disaptch 
fn c<T:A>(value:T) {

}

fn size<T:Sized>(value:T) {}
fn unsize<T:?Sized>(value:&T) {}

// generic trait and associated trait 
//  an associated type is a placeholder type defined inside a trait. Instead of making the trait generic over some type, you define that type inside the trait, and then each implementation of the trait specifies what that type is.
// Associated types help avoid generic parameters cluttering up everything. They make traits easier to read and use.
// Associated trait are used for one implemeantion of one data type for example we can not implement same trait for array if we have defined it before but in generic traits we can do it 

trait AssociatedTrait {
         type Item; 
         fn sum (&mut self)->Option<Self::Item>;
}
impl AssociatedTrait for Rust {
     type Item  = bool;
      fn sum (&mut self)->Option<bool> {
        Some(true)
      }
}

// overloading means we can know use assicated trait for different datatypes 
impl<T> Add for Point<T> where T:Add<Output=T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            p: self.p + other.p,
            s: self.s + other.s,
        }

    }
}
fn main () {
    let rust  = Rust;
    rust.execute("hello.ts");
    // if two trait have same function name then we should write in this way 
    // Trait_name::function_name(&struct_variable);
    let result:Result<String,String>  = Result::Ok("string".to_string());
    let result:Result<bool,String> = Result::Ok(true);
    let point = Point {
        p: "string".to_string(),
        s : "string".to_string()
    };
    println!("{:?}",point);
    let a = sum(3,3);
    println!("Sum {a}");
    // monomorphization when we give different types to generic data type 
     let a = (32,32);
    let b =  a.sum();
     println!("Generic Traits {b}");
     // From and Into
    
   let a =  Rust::from("Mubashir".to_string());
   println!("Name {:#?}",a);
 
   into_usage();
   // Trait bounds 
   let a: u32 = 3;
   let b : i32= -3;
   c(a);
   c(b);
   // Sized
//    Sized is known at compiler time 
//    Automatically implemented for permitive datatypes 
//    Necessary for allocating values for the stack 
 let a = 3 ;
 let arr = [3,4,2];
 size(a);
 size(arr);
 size(&arr); // because it's size is known at compiler time 
   // ? Sized 
//    Sized may not be known at compiler time 
//    Examples dynamically sized types, slices and trait objects ,dynamic traits too 

let slice = &[3,2,1];
unsize(slice);
// Static dispatch 
//function to call  known on compiler time 
// code size can be larger 
// no run time cost 
// Dynamic Dispatch 
// function to known on run time 
// run time cost 
  // Overload 
  let point =Point {
     p : 3,
     s : 4,
  };
  let points = Point {
    p : 4,
    s: 3
  };
     let addition = point + points;
     println!("Additon {:#?}",addition);
} 