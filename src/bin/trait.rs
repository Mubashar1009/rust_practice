#![allow(unused)]
#[derive(Debug, PartialEq)]
struct Score {
    score:i32
}
#[derive(Debug, PartialEq)]
struct Points {
    points:i32
}
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

fn main() {
       let a = Score{
        score : 32
       };
       let b = Points {
        points : 10
       };
       println!("{:#?}",compiler(&a, "score"));
}
