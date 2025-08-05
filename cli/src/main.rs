use std::env;
use std::process;
use cli::Config;
// for showing output in the file run: cargo run > output.txt.It generate output.txt file and error published in this file 
// for not showing output in the file use:eprintln!() instead of println!() 
fn main() {
    let args: Vec<String> = env::args().collect();
    let result = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}",err);
        process::exit(1);
    });
    // let query = &args[1];
    // let fileName = &args[2];
    println!("Query{:?}", result.query);
    println!("fileName {}", result.fileName);
   
   if let Err(e) = cli::run(result) {
    println!("Application Error {}",e);
    process::exit(1);
   }
}

