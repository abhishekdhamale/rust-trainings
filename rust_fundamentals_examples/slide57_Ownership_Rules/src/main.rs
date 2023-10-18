// fn main () {
//     let vector1 = vec![1, 2, 3];
// }		

// fn main () {
//     let vector1 = vec![1, 2, 3];
//                   ~~~~~~~~~~~~~  <---
//                                      |
//                               This memory
//                             gets `deallocated`
// }   <------------------------- here `vector1` goes out of scope

fn main () {
	 
    let vector1 = vec![1, 2, 3];  <--`vector1` is invalid variable
    let vector2 = vector1;                        beacause
        ~~~~~~~  <------------------ `vector2` is New Owner
   
    println!("{:?}",vector1);
  }
