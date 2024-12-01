use std::f64::consts::PI;
use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}

fn main() {
    println!("Select the formula to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    // Read the user selection
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    let option: u32 = option.trim().parse().expect("Please enter a number");

    match option {
        1 => {
            // Area of Trapezium
            let mut height = String::new();
            let mut base1 = String::new();
            let mut base2 = String::new();

            println!("Enter the height of the trapezium:");
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Please enter a valid number");

            println!("Enter the length of base1:");
            io::stdin().read_line(&mut base1).expect("Failed to read line");
            let base1: f64 = base1.trim().parse().expect("Please enter a valid number");

            println!("Enter the length of base2:");
            io::stdin().read_line(&mut base2).expect("Failed to read line");
            let base2: f64 = base2.trim().parse().expect("Please enter a valid number");

            let area = area_of_trapezium(height, base1, base2);
            println!("The area of the trapezium is: {}", area);
        }
        2 => {
            // Area of Rhombus
            let mut diagonal_1 = String::new();
            let mut diagonal_2 = String::new();

            println!("Enter the length of diagonal_1:");
            io::stdin().read_line(&mut diagonal_1).expect("Failed to read line");
            let diagonal_1: f64 = diagonal_1.trim().parse().expect("Please enter a valid number");

            println!("Enter the length of diagonal_2:");
            io::stdin().read_line(&mut diagonal_2).expect("Failed to read line");
            let diagonal_2: f64 = diagonal_2.trim().parse().expect("Please enter a valid number");

            let area = area_of_rhombus(diagonal_1, diagonal_2);
            println!("The area of the rhombus is: {}", area);
        }
        3 => {
            // Area of Parallelogram
            let mut base = String::new();
            let mut altitude = String::new();

            println!("Enter the base of the parallelogram:");
            io::stdin().read_line(&mut base).expect("Failed to read line");
            let base: f64 = base.trim().parse().expect("Please enter a valid number");

            println!("Enter the altitude of the parallelogram:");
            io::stdin().read_line(&mut altitude).expect("Failed to read line");
            let altitude: f64 = altitude.trim().parse().expect("Please enter a valid number");

            let area = area_of_parallelogram(base, altitude);
            println!("The area of the parallelogram is: {}", area);
        }
        4 => {
            // Area of Cube
            let mut side = String::new();

            println!("Enter the length of the side of the cube:");
            io::stdin().read_line(&mut side).expect("Failed to read line");
            let side: f64 = side.trim().parse().expect("Please enter a valid number");

            let area = area_of_cube(side);
            println!("The area of the cube is: {}", area);
        }
        5 => {
            // Volume of Cylinder
            let mut radius = String::new();
            let mut height = String::new();

            println!("Enter the radius of the cylinder:");
            io::stdin().read_line(&mut radius).expect("Failed to read line");
            let radius: f64 = radius.trim().parse().expect("Please enter a valid number");

            println!("Enter the height of the cylinder:");
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Please enter a valid number");

            let volume = volume_of_cylinder(radius, height);
            println!("The volume of the cylinder is: {}", volume);
        }
        _ => {
            println!("Invalid input! Please choose a valid option.");
        }
    }
}
