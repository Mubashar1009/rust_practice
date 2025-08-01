#![allow(unused)]
use std::collections::HashMap;
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
// A lifetime is the time during which a reference is valid. if any one variable is invalid then it throw error  
fn lifetime<'a> ( x : &'a str,  y:&'a str) ->&'a str {
           if x.len() > y.len() {
            x
           }
           else 
           {
            y
           }
}
// Pointer function 
fn push(v:&mut Vec<u32>,x:u32)->&mut Vec<u32> {
  let a =   v.push(x);
   v
}
fn adding(f:fn(&mut Vec<u32>,u32)->&mut Vec<u32>,v:& mut Vec<u32>,x:u32) ->&mut  Vec<u32> {
      let a =  f(v,x);
      a
}

// fn(),
fn function<F:Fn()>(f:F) {
    f();
}
// fnMut()
fn functionMut<F:FnMut()>(mut f:F) {
    f();
}
// fnOnce()
fn functionOnce<F:FnOnce()>(f:F) {
    f();
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
     // Iter - borrows and return a iterator that return &T
     // Into_Iter - takes ownership and return a iterator that return &T or T or &mut T 
     // Into Iter when we run a for in loop on a vector then ownership transfer so we can not use that vector again but if we use .iter() with it then we can use it 
    // iter_mut() - return &mut T
      let mut  functions  = vec![3,1,2,4];
        for v in functions.iter_mut() {
             *v += 1;
             println!("v {}",v);
        }
     // filter and map 
     let functions  = vec![3,1,2,4];
     let data  : Vec<i32>= functions.into_iter().filter(|x| *x >=2).map(|x| 2*x).collect();
     println!("Map and filter {:?}",data);
     //zip 
       let functions  = vec![3,1,2,4];
       let array = vec!["a","b","c"];
       let data : Vec<String> = array.iter().map(|x| x.to_string()).collect();
       let zipped: HashMap<String,i32> = data.into_iter().zip(functions.into_iter()).collect();
            println!("Zipped {:?}",zipped);
        let sum = vec![3,21,2];
        // folding is just like a reduce method in javascript 

        let folding  = sum.iter().fold(0,|acc,x| acc+x);
        println!("folding {}",folding);
     // Pointer Refrence 
     let mut v : Vec<u32> = vec![3,2,1];
     adding(push,&mut v,4);
     println!("Adding {:?}",v);
     // Closure - anonymous function  + capture variable in the environment 
     // let suppose we are creating a function 
     fn one_function ( x: u32,y:u32) -> u32 {
        x+y
     };
     // the above function can write as anonymous function in such a way
     let anonymous_function = |x:u32,y:u32| ->u32 {
        x + y
     }; 
     // we can write above function in such a way 
     let anonymous_function = |x,y| x+y;
     // rust is smart it can know type of it when we wil call this function but it works only for i32 not all data types 
     let z: i32 = anonymous_function(3,4);
     println!("{}",z);
     // by creating anonymous function we can use variables of the environment and can assign same function to other variable but if we have call function with one data type then we can not call with other data type 
    // Closure : 
    // closure can take refrence,mutable refrence or can tak ownership of the variable 
// borrow  immutable refrence 
    let s = "Moon".to_string();
    let f = || println!("{}",s);
    f();
    println!("{}",s);
    // here take ownership 
    let s = "Moon".to_string();
    let f = move || println!("{}",s);
       // 
       let mut s = "Mubashir".to_string();
       let f = || println!("here is function calling {}",s);
       function(f);
       let f = || println!("here is function Mut calling {}",s);
       functionMut(f);
       let f = move || println!("here is function Once calling {}",s);
       functionMut(f);

    } 