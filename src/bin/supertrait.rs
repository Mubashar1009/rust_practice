#![allow(unused)]

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
    println!("{:?}",point)
}