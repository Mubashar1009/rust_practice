use std::env;
fn main (){
    // args function run iterators on the all arguments and collect method makes collection 
    let args:Vec<String>   = env::args().collect();
    let query  = &args[1];
    let pathname = &args[2];


    println!("Argumentss {}",query);
    println!("Path Name {}",pathname)
}