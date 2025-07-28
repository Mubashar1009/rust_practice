#[allow(unused)]

struct ImportantExcerpt<'a>{
 part : &'a str
}

impl<'a> ImportantExcerpt<'a>  {
    fn return_part(&'a self, announcement:&str)-> &str {
        println!("Attention Please {}",announcement);
        self.part
    }
}
fn fullName<'a>(a:&'a str,b:&'a str)->&'a str {
    // the lifetime of output parameter is equal to shortest lifetime of the input parameter 
          if a.len() > b.len() {
            b
          }
          else {
            a
          }
}
fn main () {
    println!("Here we will study about life time ");
    // dangling refrence : it is the refrence to point to the invalid data and rust does not like dangling refrence 

    // Rules 
    // 1. Each parameter that is a refrence gets its own lifetime parameter
    // 2. if there is exactly one input lifetime parameter,that lifetime is assigned to all output parameters 
    // 3. if there are multiple input lifetime parameter, but one of them is &self and &mut self the lifetime of self is assigned to all output lifetime paramter 

    // static lifetime 
    // static lifetime could live as long as long as the duration of the program and all string literal have static lifetime 
    let a = "Mubashir";
    let b = "Moon";
    let c = fullName(a,b);
    println!("LifeTime {}",c);
    let c = ImportantExcerpt {
        part : "Mubashir Shakeel"
    };
    let d = c.return_part("Mubashir Shakeel");


}