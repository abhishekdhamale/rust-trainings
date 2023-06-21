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
    fn div(&self) -> i32 {
        self.first_num / self.second_num
    }
    fn mul(&self) -> i32 {
        self.first_num * self.second_num
    }
}
fn main() {
    println!("Output of Add: {}", Data {first_num:2, second_num: 2}.add());
    println!("Output of Sub: {}", Data {first_num:4, second_num: 2}.sub());
    println!("Output of Div: {}", Data {first_num:10, second_num: 2}.div());
    println!("Output of Mul: {}", Data {first_num:2, second_num: 2}.mul());
}