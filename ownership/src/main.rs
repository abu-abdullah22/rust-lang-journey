// 1. each value in rust has a variable that’s called its owner.
// 2. there can only be one owner at a time.
// 3. when the owner goes out of scope, the value will be dropped.

// example of the first rule 
// fn main() {
//    let s1 = String::from("Rust");
//    let len = calculate_length(&s1);
//    println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// example of the second rule
// fn main() {
//     let s1 = String::from("Rust");
//     let s2 = s1;
//     // println!("{}", s1); // this will cause a compile-time error
//     println!("{}", s2); // this works
// }

// fn main () {
//     let s1 : &str = "Rust";
//     let s2 = s1; 
//     println!("s1 = {}, s2 = {}", s1, s2); 
// }


// third rule example
fn main(){
    let s1 = String::from("Rust");
}

// s1 goes out of scope here and the memory is freed
// println!("{}", s1); // this will cause a compile-time error
