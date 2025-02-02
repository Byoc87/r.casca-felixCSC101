use std::io;

fn main() {
    // Constants for carbon emissions per hour (in kg of CO2)
    const LAPTOP_CO2: f64 = 0.01;  // kg CO2 per hour
    const PHONE_CO2: f64 = 0.002;  // kg CO2 per hourp
    const CAR_CO2: f64 = 0.2;      // kg CO2 per hour

    // Vector to store the usage data for 3 students
    let mut student_data: Vec<(u32, u32, u32)> = Vec::new();

    // Collecting data for each student
    for i in 1..=3 {
        println!("Enter data for Student {}:", i);

        // Input the hours of laptop usage
        let laptop_hours = get_input("Enter the number of hours using the laptop: ");
        
        // Input the hours of phone usage
        let phone_hours = get_input("Enter the number of hours using the phone: ");
        
        // Input the hours of car usage
        let car_hours = get_input("Enter the number of hours using the car: ");

        student_data.push((laptop_hours, phone_hours, car_hours));
    }

    // Calculating and displaying the carbon emissions for each student
    for (i, (laptop_hours, phone_hours, car_hours)) in student_data.iter().enumerate() {
        let laptop_emissions = *laptop_hours as f64 * LAPTOP_CO2;
        let phone_emissions = *phone_hours as f64 * PHONE_CO2;
        let car_emissions = *car_hours as f64 * CAR_CO2;

        let total_emissions = laptop_emissions + phone_emissions + car_emissions;

        println!("\nStudent {}:", i + 1);
        println!("Laptop CO2 Emissions: {:.2} kg", laptop_emissions);
        println!("Phone CO2 Emissions: {:.2} kg", phone_emissions);
        println!("Car CO2 Emissions: {:.2} kg", car_emissions);
        println!("Total CO2 Emissions: {:.2} kg", total_emissions);
    }
}

// Function to handle user input and ensure it's a valid positive integer
fn get_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
