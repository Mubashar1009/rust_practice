use std::fs;
use std::error::Error;
pub fn run (config:Config)->Result<(),Box<dyn Error>>{
    // for using environment variables export environmentVaraible=boolean
    // for unset environment variables unset environmentVaraible
    // example let cas_insensitive = env.var("ENVIRONMENTVARIABLE").is_err();
    // it will return true or false 
    
    let readFile = fs::read_to_string(config.fileName)?;
    println!("Reading file\n {}", readFile);
    Ok(())
}
pub struct Config {
   pub  query: String,
   pub  fileName: String,
}

impl Config {
  pub  fn new(args: &[String]) -> Result<Config,&str> {
        if args.len() < 3 {
            return Err("Something Went wrong ")
        }
        let query = args[1].clone();
        let fileName = args[2].clone();
        Ok(Config { query, fileName }) 
    }
    
}

fn search<'a>(query:&str,content:&'a str) ->Vec<&'a str> {
    let mut result = Vec::new();
          for line in content.lines(){
          if  line.contains(query) {
            result.push(line);
          }
          }
          result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let query = "test";
       let fileName = "test";
       assert_eq!(vec!["test"],search(query,fileName));
    }
   
}

