fn call_function< F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}
 
fn return_function() -> Box < dyn Fn(i32) -> i32> {
    Box::new(|x: i32| -> i32 { x * x })
}
 
fn main(){
    // Closure
    let square = |x: i32| -> i32 { x * x };
    println!("The square of 2 is {}", square(2));
 
    // Closure can be passed as an argument to a function
    println!("The square of 2 is {}", call_function(square, 2));
 
    // Closure can be returned as a result from a function:
    let square = return_function();
}