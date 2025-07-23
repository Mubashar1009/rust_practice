#[allow(unused)]

use::std::{fs::File,io::{Error,Read}};

fn reading_file(path:&str) ->Result<String,Error> {
      let mut file = File::open(path)?;
      let mut content = String::new();
      file.read_to_string(&mut content)?;
      Ok(content)
}

fn main () {
    println!("Hi i am learning about ? operator");
    // ? operator used in mostly Result or Options to minimize and clean code 
    // unwrap() is used when there is 100% sure that there is no error otherwise we should not use it 
    let file = "output.txt";
    let _contents = match reading_file(file) {
        Ok(data) => println!("data {data}"),
        Err(r) =>println!("{r}"),
    };
} 