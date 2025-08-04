use std::env;
use std::fs;
use std::process;
use std::error::Error;
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
   
   if let Err(e) = run(result) {
    println!("Application Error {}",e);
    process::exit(1);
   }
}
fn run (config:Config)->Result<(),Box<dyn Error>>{
    let readFile = fs::read_to_string(config.fileName)?;
    println!("Reading file\n {}", readFile);
    Ok(())
}
struct Config {
    query: String,
    fileName: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config,&str> {
        if args.len() < 3 {
            return Err("Something Went wrong ")
        }
        let query = args[1].clone();
        let fileName = args[2].clone();
        Ok(Config { query, fileName }) 
    }
    
}