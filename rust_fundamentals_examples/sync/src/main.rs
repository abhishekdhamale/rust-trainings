// SEND TRAIT EXAMPLE
// use std::thread;
	 
// 	fn main(){
	 
// 		let data = vec![1, 2, 3];
// 		thread::spawn(move || {
// 			println!("{:?}", data);
// 		});
// 	}


// SYNC TRAIT EXAMPLE

use std::sync::Arc;
use std::thread;
 
fn main(){
    let data = Arc::new(vec![1, 2, 3]);
    let data_clone = data.clone();
    
    thread::spawn(move || {
        println!("{:?}", data_clone);
    });
    
    println!("{:?}", data);
}