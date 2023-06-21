fn accepts_str(s: &str) {
    // code ..
}
 
fn main(){
    let s = String::from("hello");
    accepts_str(&s);
}

	// // creating a String using the String::new
	// let s = String::new(); 
	// //creating a String using the String::from function
	// let s = String::from("hello"); 
	// //converting a `&str` to a `String` using the to_string method
	// let s = "hello".to_string();

    // //creating an &str from a string literal
	// let s = "hello";
	// //creating an &str from a String:
	// let s = String::from("hello");
	// let str_slice = &s[0..2];


    // let s: String = String::from("hello world");
	 
	// let world: &str = &s[6..11];

// OPERATIONS ON STRINGS

// let s1 = String::from("hello world");
// let mut s2 = String::from("rust is awesome");
// let s3 = s1 + &s2;             // Concat using the `+` operator
// let s4 = format!("{}{}",s1,s2);// Concat using the format! macro
// s2.push_str(", & memory safe");// Appending using the push_str
// let slice = &s[0..2];          // Slicing using string indices
 
// for c in "hello".chars() {     // Iterating over the string
// 	println!("{}", c);
// }
// println!("Length:{}", s.len());// Finding the length of a string
// if(s1.starts_with("he")){}     // Check if starts with a prefix
// if(s1.ends_with("ld")){}       // Check if ends with a suffix