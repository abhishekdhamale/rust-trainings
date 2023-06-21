fn min< T:PartialOrd > (a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
 
fn main(){
    let x = min(5, 10);              // integer
    let y = min(5.0, 10.0);          // float
    let z = min("hello", "world");   // string
    
    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");
}