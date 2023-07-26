struct Point {
    x: i32,
    y: i32,
}
 
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
fn main(){
    let p = Point::new(1, 2);
}