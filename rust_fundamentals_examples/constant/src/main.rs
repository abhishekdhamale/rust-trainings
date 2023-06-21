	struct Circle {
		radius: f64,
	}
fn main() {
    impl Circle {
		const PI: f64 = 3.14159;
	 
		fn new(radius: f64) -> Circle {
			Circle { radius }
		}
		fn calculate_area(&self) -> f64 {
			Circle::PI * self.radius * self.radius
		}
	}	
}
	