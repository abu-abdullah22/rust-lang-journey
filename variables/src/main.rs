// a variable is immutable by default
// to make a variable mutable, use the `mut` keyword
fn main() {
    let mut x = 5 ; // added mut so that x is mutable
    println!("x = {}",x);
    x = 10 ; // now this is valid, but if mut was not used, this would cause a compile-time error
    println!("The value of x is: {}", x);
}


