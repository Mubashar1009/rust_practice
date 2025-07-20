#![allow(unused)]

fn closure_many() -> impl Fn(i32) -> i32 {
    let a = 3;
    // here we need this move because once closure will return then we can not access a so we transfer ownership
    move |v| v + a
}
fn closure_mut() -> impl FnMut(i32) -> i32 {
    let a = 2;
    // here we need this move because once closure will return then we can not access a so we transfer ownership
    move |v| v + a
}
fn closure_once() -> impl FnOnce(i32) -> i32 {
    let a = 2;
    // here we need this move because once closure will return then we can not access a so we transfer ownership
    move |v| v + a
}

#[derive(Debug)]
struct RecursiveDataType {
    val: i32,
    left: Option<Box<RecursiveDataType>>,
}
// enum List {
//   Cons(i32,Box<List>),
//   Nil
// }
use std::{cell::RefCell, rc::Rc,rc::Weak};
// Weak refrence 
// refrence to data that may be live or dealocated 
//increment weak_count()
//data can be dropped even if weak_count>0
// how to use weak refrence 
// Rc::downgrade 
// create weak refrence 
// cannot access behind the refrence 
// weak_count increases by 1
// create weak refrence 
// Rc::upgrade 
// upgrades to weak to strong refrence 
// strong_count increases 
// can access behind the refrence 
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
#[derive(Debug)]
struct WeakCount {
  val : u32,
  neighbors : RefCell<Vec<Rc<WeakCount>>>
}
fn main() {
    let f = closure_many();
    let mut f = closure_mut();
    let f = closure_once();
    println!("{:#?}", f(1));
    // smart pointer, it is used when there are no size define on the compile time
    let a = Box::new(3);
    // access value of the smart pointer
    let b = *a;
    println!("{}", b);
    let a = RecursiveDataType {
        val: 32,
        left: Some(Box::new(RecursiveDataType {
            val: 21,
            left: None,
        })),
    };
    println!("{:?}", a.left.unwrap().val);
    // list
    // let listing = Cons(2,Box::new(Nil));
    // let a  = Cons(3,Box::new(listing));
    // Box takes ownership so we can not do this but by using refrence we can do it
    // let  b = Cons(4,Box::new(listing));
    // Rc refrence count
    // use to share ownership for reading prupose only
    //keep track of the number of the refrences to the value wrapped in RC
    // refrence count is increased when we RC clone
    // decrease count when cloned RC is dropped
    // cloning a RC count never perform a deep copy

    let list = Rc::new(Cons(1, Rc::new(Nil)));
    println!("{}", Rc::strong_count(&list));
    let a = Cons(2, Rc::clone(&list));
    println!("{} {:?}", Rc::strong_count(&list), list);
    let mut listing = &a;
    while let Cons(v, tail) = listing {
        println!("{v} -> ");
        listing = &**tail;
    }
    // RefCell: RefCell allows interior mutability, meaning we can mutate data even when it is accessed through an immutable reference.
    // Without RefCell (or similar types like Cell), this would not be possible in Rust due to its borrowing rules.
     let a = RefCell::new("Mubashir".to_string());
       {
         let mut b  = a.borrow_mut();
          
            *b = "Moon".to_string();
            println!("{:#?}",a);
       }
       println!("{:#?}",a);
       // Memory Leak
       // Allocated memory is no longer accessible but it is not freed 
       // variables is no longer accessible but it still exists in memory 
       let v0 = Rc::new(WeakCount {
        val : 0,
        neighbors : RefCell::new(vec![])
       });
       let v1 = Rc::new(WeakCount {
        val : 1,
        neighbors : RefCell::new(vec![])
       });
      {
          let mut a = v0.neighbors.borrow_mut();
       a.push(Rc::clone(&v0));
            let mut b = v1.neighbors.borrow_mut();
       b.push(Rc::clone(&v1));
      }
      println!("{:?}",Rc::strong_count(&v0));
        println!("{:?}",Rc::strong_count(&v1));
        std::mem::drop(v0);
           // here is memory leak after dropping v0 this is still exist in the memory because it only drop when strong_count is equal to 0 
           // for removing it we use weak refrence 

        println!("{:?}",Rc::strong_count(&v1));
        //if Rc::strong_count() is equal to zero 
  }
