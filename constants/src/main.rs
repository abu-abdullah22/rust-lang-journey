// constant and its usage in Rust 
// A constant is a value that is bound to a name and is not allowed to change.
// Constants are always immutable and must have a type annotation.

fn main() {
    let x : i32 = 10; 
    println!("The maximum points is: {}", x);

    const Y : u32 = 5;
    println!("The value of Y is: {}", Y);

    area_of_circle(2.0);
    println!("The area of circle with radius 2.0 is: {}", area_of_circle(2.0));
}
// you can declare a const out side of a function 
const PI : f64 = 3.14; 
fn area_of_circle(radius: f64) -> f64 {
    PI * radius * radius
}