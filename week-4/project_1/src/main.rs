use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first value");
    io::stdin().read_line(&mut input1).expect("Value is an integer");
    let a:f32 = input1.trim().parse().expect("Not a valid integer");

    println!("Enter second value");
    io::stdin().read_line(&mut input2).expect("Value is an integer");
    let b:f32 = input2.trim().parse().expect("Not a valid integer");

    println!("Enter third value");
    io::stdin().read_line(&mut input3).expect("Value is an integer");
    let c:f32 = input3.trim().parse().expect("Not a valid integer");

    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Your values are {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = (-b) / (2.0 * a);
        println!("There is exactly one real root {}", root);
    } else {
        println!("There are no real roots");
    }

}