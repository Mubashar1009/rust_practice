#![allow(unused)]

fn closure_many()->impl Fn(i32) -> i32 {
             let a = 3;
             // here we need this move because once closure will return then we can not access a so we transfer ownership
           move |v| v+a
}
fn closure_mut()->impl FnMut(i32) ->  i32  {
    let a = 2;
    // here we need this move because once closure will return then we can not access a so we transfer ownership
  move |v| v+a
}
fn closure_once()->impl FnOnce(i32) ->  i32 {
    let a = 2;
    // here we need this move because once closure will return then we can not access a so we transfer ownership
  move |v| v+a
}

#[derive(Debug)]
struct RecursiveDataType {
    val : i32,
    left : Option<Box<RecursiveDataType>>

}
fn main() {
    let f = closure_many();
    let mut f  = closure_mut();
    let f = closure_once();
    println!("{:#?}",f(1));
    // smart pointer, it is used when there are no size define on the compile time 
    let a = Box::new(3);
    // access value of the smart pointer 
    let  b= *a;
    println!("{}",b);
    let a = RecursiveDataType {
        val : 32,
        left : Some(Box::new(RecursiveDataType  {
            val : 21,
            left : None
        }))
        
    };
    println!("{:?}",a.left.unwrap().val);
 
}