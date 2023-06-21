let vector2 = &vector1;
	              ^ -------------
	someFunction(&vector1);      |---> Reference
	             ^ --------------


fn main () {
	  let mut vector1 = vec![1, 2, 3];
	      ^^^  --------------------
	  let vector2 = &mut vector1;  |---> Explicitly mentioned  
	                 ^^^ ----------      as `mutable`
	                                     using `mut` keyword
	}


fn main () {
	   let mut _vector1 = vec![1, 2, 3];
	   let _vector2 = &mut _vector1;
	   let _vector3 = &mut _vector1;
	   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ---> NOT ALLOWED
	}


