pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width : i32,
    height : i32
}
impl Rectangle {
    fn check(&self,value:&Rectangle) -> bool {
        self.width > value. width && self.height > value.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
     #[test]
    fn panic() {
       panic!("test fail");
    }
     #[test]
    fn check_rectangle() {
       let larger = Rectangle { 
        width : 5,
        height : 8
       };
       let smaller = Rectangle {
        width : 3,
        height : 2
       };
       assert!(larger.check(&smaller));
    }
}
