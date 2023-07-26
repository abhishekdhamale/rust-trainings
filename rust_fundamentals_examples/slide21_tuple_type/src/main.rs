fn main() {
    // creating tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let gfg: (&str, &str, &str) = ("Apple", "For", "Bananas");
 
    // accessing tuple data using positional argument
    println!("{} {} {}", gfg.0, gfg.1, gfg.2);
 
    // creating another tuple
    let article = ("abc", "xyz", 14,12,2020);
    let (a,b,c,d,e) = article;
 
    // accessing tuple using variables
    println!("This written by {} at {} on {}/{}/{}", b,a,c,d,e);
}			
