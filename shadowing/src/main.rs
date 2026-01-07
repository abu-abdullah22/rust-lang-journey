// shaadowing is a feature in Rust that allows you to declare a new variable with the same name as a previous variable. 
// The new variable shadows the previous variable, meaning that the previous variable is no longer accessible.
// This can be useful for transforming a value while keeping the same name.
// In this example, we declare a variable x with the value 5, then we shadow it by declaring a new variable x that is equal to the previous x plus 1.
fn main() {
    let x = 5;
    let x = x + 1;
    {
        println!("The value of x in the inner scope is: {}", x*2);
    }

    println!("value of x is still : {}", x);

   shadowing_string();
}
// The output of this program will be "The value of x in the inner scope is: 6" because the inner scope accesses the shadowed variable x, which has the value 6.
// in string is not copied when shadowed but moved.
// In this example, we declare a variable s with the value "hello", then we shadow it by declaring a new variable s that is equal to the previous s plus ", world!".
fn shadowing_string() {
    let s = String::from("hello");
    let s = s + ", world!";
    {
        println!("The value of s in the inner scope is: {}", s);
    }
}




