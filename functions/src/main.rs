fn main() {
    tell_height(5);
    human_id("Alice", 30, 5.2);
    println!("the product of y and z is {}", X);
    // let sum: i32 = add(10, 20);
    println!("the sum of 10 and 20 is {}", add(10, 20));

    let height: f64 = 1.78;
    let weight: f64 = 50.0;
    let bmi: f64 = bmi_calc(height, weight);
    println!("The BMI is {:.2}", bmi);

}

fn tell_height (height: i32) {
    println!("I am {} feet tall.", height);
}

fn human_id(name: &str, age: i32, height: f32) {
    println!("My name is {}, my age is {}, my height is {} feet.",name, age, height);
}

// expression and statement 
const X : i32 = {
    let y: i32 = 5;
    let z :i32 = y + 2;

    y * z
};


fn add (a: i32, b: i32) -> i32 {
    a + b 
}

fn bmi_calc(height: f64, weight: f64) -> f64 {
    weight / (height * height)
}

