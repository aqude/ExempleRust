use std::fmt;
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

fn main() {
let xy = Point { x: 11.0, y: 212.0 };
println!("Display: {}", xy); // 1
println!("Debug: {:?}", xy); // 2
}

//1 Display: x: 11 y: 212
//2 Debug: Point { x: 11.0, y: 212.0 }
