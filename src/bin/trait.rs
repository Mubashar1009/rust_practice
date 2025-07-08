#![allow(unused)]

#[derive(Debug, PartialEq)]
struct Score {
    score:i32
}
#[derive(Debug, PartialEq)]
struct Points {
    points:i32
}
#[derive(Debug, PartialEq)]
struct Dog;
struct Cat;
//Traits in Rust are like interfaces in other languages (e.g., Java or TypeScript)  If a type implements this trait, it must provide these methods.
trait Compiler { // here we can use only simple calling function wit return type without any inner function code 
    // if we does not add inner function code for any struct then it would run this trait's inner function 
    fn compiler(&self,file_name:&str){
        format!("Compiler {file_name}");
        
    }
}
impl Compiler for Score {
     fn compiler(&self,file_name:&str){
        println!("Scoring {file_name}");
    }
}
impl Compiler for Points {
     fn compiler(&self,file_name:&str){
        format!("Points {file_name}");
    }
}

 fn compiler(lang:&impl Compiler,file_name:&str){
          return lang.compiler(file_name)
    }
// Dynmainc Trait it does not known until runtime 
// we can not directly return refrence so we use Box
fn compiler_dyn(lang:&impl Compiler,file_name:&str){
    return lang.compiler(file_name)
}
trait Animal { // here we can use only simple calling function wit return type without any inner function code 
    // if we does not add inner function code for any struct then it would run this trait's inner function 
    fn animal(&self){
    println!("Dynamic Animal")
        
    }
}

impl Animal for Cat {
    fn animal(&self){
       println!("Cat");
   }
}

impl Animal for Dog {
    fn animal(&self){
       println!("Dog");
   }
}

    fn condition(rand:u32)->Box<dyn Animal>{
       if rand > 32 {
        Box::new(Cat)
       }
       else {
Box::new(Dog)
       }
  }
  
fn main() {
       let a = Score{
        score : 32
       };
       let b = Points {
        points : 10
       };
       println!("{:#?}",compiler(&a, "score"));
      let a   = condition(33);
      println!("{:#?}",a);
     
}
