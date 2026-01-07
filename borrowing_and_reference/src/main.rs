// references allow you to borrow values without taking ownership of them
// references are immutable by default, meaning you cannot change the value they point to
// to create a mutable reference, you need to use the `mut` keyword
// you can have multiple immutable references to a value, but only one mutable reference at a time
// this prevents data races at compile time     
// references must always be valid, meaning they cannot outlive the value they point to
// Rust's borrow checker enforces these rules at compile time
// references are created using the `&` operator, and dereferenced using the `*` operator
// here's an example of using references in Rust:

// fn main() {
//     let mut _x: i32 = 10;
//     let _y: &mut i32 = &mut _x;
//     // println!("The value of x is: {}", _x);
//     // println!("The value of y is: {}", _y);
//     // *_y += 1; 
//     println!("The value of y is: {}", _y);
// }


fn main (){
    let x = 10;
    let y = &x;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);     
}


