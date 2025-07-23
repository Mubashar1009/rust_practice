#[allow(unused)]
// concurrency programming 
// thread : thread are mostly useful when computation that can be done in parallel 
// channel 
// Asyn/await programming 
use::std::thread;
use::std::thread::JoinHandle;
use::std::time::Duration;
fn main () {
   let join:JoinHandle<()>  = thread::spawn (|| {
        for i in 0..5 {
            println!("{i}");
            thread::sleep(Duration::from_millis(100));
        }
    });
   join.join().unwrap();
   let vector = vec![3,2,3];
  let h =  thread::spawn(move|| {
      println!("{:?}",vector);
   });
    h.join().unwrap();
    let h =  thread::spawn(move|| {
          return 3
     });
     match h.join(){
        Ok(val)=>println!("{val}"),
        Err(err)=>{} 
     }
}