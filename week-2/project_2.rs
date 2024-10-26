fn main(){
    let toshiba:f64 = 450000.0;
    let mac:f64 = 1500000.0;
    let hp:f64 = 750000.0;
    let dell:f64 = 2850000.0;
    let acer:f64 = 250000.0;

    let sum:f64 = (toshiba * 2.0) + (mac * 1.0) + (hp * 3.0) + (dell * 3.0) + (acer * 1.0) ;
    println!("Total sum is {}", sum);

    let toshiba_qty:f64 = 2.0;
    let mac_qty:f64 = 1.0;
    let hp_qty:f64 = 3.0;
    let dell_qty:f64 = 3.0;
    let acer_qty:f64 = 1.0;

    let quantity = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty ;
    println!("Total quantity is {}", quantity);

    let average = sum/quantity;
    println!("Average is {}", average)
}