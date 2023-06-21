// IMPLEMETING TRAITS FOR EXISTING TYPES
trait PrintInfo {						//Define a trait called `PrintInfo`
    fn print_info(&self);	
}
 
impl PrintInfo for i32 {		// Implement `PrintInfo` for the built-in type `i32`
    fn print_info(&self) {
        println!("This is an integer: {}", self);
    }
}
 
fn main() {
    let num: i32 = 42;
    num.print_info(); 				// Output: "This is an integer: 42"
}

// ACCESS METHODS FROM THE SAME TRAIT

// trait Calculator {
//     fn add(&self) -> u32;
//     // We can provide default method definitions.
//     fn get_result(&self) {
//         println!("Result of Add() is {}", self.add());
//     }
// }

// PASSING TRAIT AS A FUNCTION PARAMETER

// pub fn calculate(item: impl Calculator) {
//     println!("Addition {}", item.add());
// }

// TRAIT BOUND CONCEPT

// pub fn calculate< T: Calculator> (item: T) {
//     println!("Addition {}", item.add());
// }

