use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("Hi there, Welcome to globacom operations....");
    println!("Kindly enter what kind of user you are.\n
This below is a guideline to help you with your selection\n
     Enter 1 if you are an Administrator,\n
     Enter 2 if you are a Project Manager,\n
     Enter 3 if you are an Employee,\n
     Enter 4 if you are a Customer,\n
     Enter 5 if you are a Vendor
");

let matches = vec!["1", "2", "3", "4", "5"];
let mut response = String::new();
loop {
    io::stdin().read_line(&mut response).expect("Unable to get your user response");
    response = response.trim().to_lowercase();
    if matches.contains(&&response.as_str()) {
        break;
    }
    else {
        println!("You have not entered a valid input.");
    }
    response.clear();
}

let response = response.as_str();
if response == "1"{
    display("globacom_dbase.sql");
} 
else if response == "2"{
    display("project_tb.sql");
}
else if response == "3"{
    display("staff_tb.sql");
}
else if response == "4"{
    display("customer_tb.sql");
}
else{
    display("dataplan_tb.sql");
}

println!("Thank you so much for using this program.");
}

fn display(path: &str) {
    let mut file = File::open(path).expect("Unable to retrieve file contents.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}