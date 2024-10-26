fn main(){
    let p:f64 = 210000.0;
    let r:f64 = 5.0;
    let t:i32 = 3;

    let a = p * (1.0 - (r/100.0)).powi(t);
    println!("Amount is {}", a);

    let ci = a - p ; 
    println!("Compound interest is {}", ci);
}