use adder;

mod common;
#[test]
fn test(){
    // to run specific integration_test run: cargo test --test file_name
    // each file in tests folder act as a one integration test 
    // subdirectory in the tests folder does not run if we want that first run specific test then run other test put that test into sub_directory and file name should be mod.rs
   common::set_up();
   let result = adder::add(3,4);
   assert_eq!(7,result);
}