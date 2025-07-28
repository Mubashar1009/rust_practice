#[allow(unused)]
use thiserror::Error;
use std::io;
use::std::{fs::File,io::{Error,Read}};
use anyhow::{Context, Result};
use anyhow::anyhow;
#[derive(Debug,Error)]
pub enum MyError {
    #[error("Internal Server Error")]
    ServerError,
    #[error("Validation Error filed_name: {} Failer: {}",.field_name,.failer_str)]
    ValidationError {
        field_name : String,
        failer_str : String,
    },
    #[error("Network Error {}",.0)]
    NetworkError(io::Error)
}


fn reading_file(path:&str) ->Result<String> {
      let mut file = File::open(path).context("file is not present ")?;
      let mut content = String::new();
      file.read_to_string(&mut content).with_context(||  anyhow!("Something went wrong "))?;
      Ok(content)
}

fn main () {
    println!("Hi i am learning about ? operator");
    // ? operator used in mostly Result or Options to minimize and clean code 
    // unwrap() is used when there is 100% sure that there is no error otherwise we should not use it 
    let file = "src/bin/output.txt";
    let _contents = match reading_file(file) {
        Ok(data) => println!("data {data}"),
        Err(r) =>println!("There is error {:?}",r),
    };
    println!("{}",MyError::ServerError);
    println!(" {}",MyError::ValidationError{
        field_name : "username".to_string(),
        failer_str : "username can not be empty".to_string()
    });
    println!("{}", MyError::NetworkError(std::io::Error::new(std::io::ErrorKind::ConnectionReset,"Network Error")));
} 