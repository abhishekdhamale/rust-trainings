fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
 
fn main() {
    let s1 = "Hello";
    let s2 = "World";
    let result = longest_string(s1, s2);
    println!("Longest string: {}", result);
}