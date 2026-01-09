// there are 3 types of loops in Rust: `loop`, `while`, and `for`


#![allow(warnings)]
fn main() {
    // loop 
    loop {
        println!("This will print forever unless we break out of the loop.");
        break; // we use break to exit the loop
    }

    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // we can also return a value from a loop
        }
    };
    println!("The result is {}", result);

}