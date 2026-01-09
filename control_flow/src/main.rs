// Control Flow with If-Else Statements
// This program checks if a person is old enough to drive a car.
// The legal driving age is set to 18.
// If the person's age is 18 or older, it prints a message allowing them to drive.
// Otherwise, it prints a message denying them the ability to drive.

#![allow(warnings)] // To suppress warnings for unused variables
fn main() {
    // let age : u16 = 18;

    // if age >= 18 {
    //     println!("You can drive a car!");
    // } else {
    //     println!("You can't drive a car!");
    // }

    let number = 10;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let con = true;
    let n = if con { 7 } else { 14 };
    let m = if con {
        7
    } else {
        "14"
    };
    println!("The value of n is: {n}");
}
