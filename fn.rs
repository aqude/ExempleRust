fn result(x: i32, y: i32, operation: fn (i32, i32) -> i32) -> i32 {
    let result = operation(x, y);
    result
}


fn main() {
    let sum: fn (i32, i32) -> i32 = |a, b| a + b; let min: fn (i32, i32) -> i32 = |a, b| a - b;
    let psum = result(10, 5, sum);
    let pmin = result(10, 5, min);
    println!("print: {}", psum);
    println!("print: {}", pmin);
}
