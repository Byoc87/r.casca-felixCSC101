use std::io;

fn main() {
let mut input1 = String::new();
let mut input2 = String::new();

println!("Is the employee experienced: (yes/no)");
io::stdin().read_line(&mut input2).expect("Ivalid Input");
let is_experienced = input2.trim().to_lowercase() == "yes";

println!("Input age of the employee");
io::stdin().read_line(&mut input1).expect("Incorrect variable inputed");
let age:i32 = input1.trim().parse().expect("Invalid input");

let incentive:u32;

if is_experienced {
    if age > 40 {
        incentive = 1_560_000;
        println!("The Incentive is N{}",incentive);
    } else if age >= 30 {
        incentive = 1_480_000;
        println!("The Incentive is N{}",incentive);
    } else if age < 28{
        incentive = 1_300_000;
        println!("The Incentive is N{}",incentive);
    } else if age < 40 {
        incentive = 1_480_000;
        println!("The Incentive is N{}",incentive);
    } else {
        incentive = 100_000;
        println!("The Incentive is N{}",incentive);
    }
   
}

}