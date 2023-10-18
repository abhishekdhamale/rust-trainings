struct Rectangle {
    width: i32,
    height: i32,
}
 
impl Rectangle {
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> i32 {
        self.width * self.height
    }
}