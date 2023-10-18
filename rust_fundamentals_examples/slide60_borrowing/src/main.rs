

fn main () {
	  let mut vector1 = vec![1, 2, 3];
	      ^^^  --------------------
	  let vector2 = &mut vector1;  |---> Explicitly mentioned  
	                 ^^^ ----------      as `mutable`
	                                     using `mut` keyword
	}





