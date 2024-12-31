use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let file_name = "nigerian_breweries_drinks.txt";

    // Data to save
    let drinks = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        (
            "Non-Alcoholic",
            vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"],
        ),
    ];

    // Create and open the file
    let mut file = File::create(file_name)?;

    // Write data to the file
    writeln!(file, "Nigerian Breweries - High-Quality Categories of Drinks")?;
    writeln!(file, "-----------------------------------------------")?;

    for (category, brands) in drinks {
        writeln!(file, "{}:", category)?;
        for brand in brands {
            writeln!(file, "  - {}", brand)?;
        }
        writeln!(file)?; // Add a blank line between categories
    }

    println!("Data saved to {}", file_name);

    Ok(())
}