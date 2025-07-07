struct Name {
    first_name: String,
    second_name: String,
}
// we can use this mode into naming mode and naming mode into this mode 

pub fn show_name() {
    let a = Name {
        first_name: "Mubashir".to_string(),
        second_name: "Shakeel".to_string(),
    };
     println!("Full name: {} {}", a.first_name, a.second_name);
    
}

 pub mod naming {
     use   super::super::module1;
     pub fn show_naming () {
        module1::first_function();
     }

     }