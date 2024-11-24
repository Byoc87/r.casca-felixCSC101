use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

let p = "poundo yam";
let f = "fried rice";
let a = "amala and ewedu";
let e = "eba and egusi";
let w = "rice and stew";

    println!("Poundo yam-  #3200");
    println!("Fried Rice & Chicken-  #3000");
    println!("Amala & Ewedu soup-  #2500");
    println!("Eba & Egusi Soup-  #2000");
    println!("White Rice & Stew-  #2500");

    println!("Press p to select poundo");
    println!("Press f to select fried rice");
    println!("Press a to select amala and ewedu");
    println!("Press e to select eba and egusi");
    println!("Press w to select white rice ad stew");


println!("\nChoose the food you want from the options listed above by typig in the initials represnted by the food: ");
std::io::stdin().read_line(&mut input1).expect("Invalid input");
let a:&str = input1.trim();

println!("\nInput the amount of quantity wanted: ");
std::io::stdin().read_line(&mut input2).expect("Invalid input");
let b:i32 = input2.trim().parse().expect("Invalid Input");

 if a=="p"{
    let price = 3200 * b;
    if price >10000 {
        let discount_price = price - (price * 5/100);
        println!("Your total amount after discount is N{}",price);
    }
    else{
        println!("your total amount is N{}",price);
    }
 }

 if a=="f"{
    let price = 3000 * b;
    if price >10000 {
        let discount_price = price - (price * 5/100);
        println!("Your total amount after discount is N{}",price);
    }
    else{
        println!("your total amount is N{}",price);
    }
 }

 if a=="a"{
    let price = 2500 * b;
    if price >10000 {
        let discount_price = price - (price * 5/100);
        println!("Your total amount after discount is N{}",price);
    }
    else{
        println!("your total amount is N{}",price);
    }
 }

 if a=="e"{
    let price = 2000 * b;
    if price >10000 {
        let discount_price = price - (price * 5/100);
        println!("Your total amount after discount is N{}",price);
    }
    else{
        println!("your total amount is N{}",price);
    }
 }

 if a=="w"{
    let price = 2500 * b;
    if price >10000 {
        let discount_price = price - (price * 5/100);
        println!("Your total amount after discount is N{}",price);
    }
    else{
        println!("your total amount is N{}",price);
    }
 }
}
