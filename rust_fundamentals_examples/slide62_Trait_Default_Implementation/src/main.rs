pub trait Calculator {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn get_result(&self) {
        println!("The result of Addition is {}", self.add());
    }
}
 
struct Data {
    first_num: i32,
    second_num: i32
}
 
impl Calculator for Data {
    fn add(&self) -> i32 {
        self.first_num + self.second_num
    }
 
    fn sub(&self) -> i32 {
        self.first_num - self.second_num
    }
}
 
fn main() {
    Data {first_num:2, second_num: 2}.get_result();
    println!("Output of Sub: {}", Data {first_num:4, second_num: 2}.sub());
}