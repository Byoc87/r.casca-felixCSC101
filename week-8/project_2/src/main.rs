// Define a struct to represent a Candidate
#[derive(Debug)]
struct Candidate {
    name: String,
    years_of_experience: u32,
}

// Function to find the candidate with the highest years of experience
fn find_highest_experience(candidates: Vec<Candidate>) -> Option<Candidate> {
    // Using an iterator to find the maximum years of experience
    candidates.into_iter().max_by_key(|c| c.years_of_experience)
}

fn main() {
    // Create some sample candidates
    let candidates = vec![
        Candidate {
            name: String::from("Alice"),
            years_of_experience: 5,
        },
        Candidate {
            name: String::from("Bob"),
            years_of_experience: 8,
        },
        Candidate {
            name: String::from("Charlie"),
            years_of_experience: 12,
        },
        Candidate {
            name: String::from("Diana"),
            years_of_experience: 9,
        },
    ];

    // Find the candidate with the highest experience
    match find_highest_experience(candidates) {
        Some(candidate) => {
            println!("The candidate with the highest experience is {} with {} years.",
                     candidate.name, candidate.years_of_experience);
        },
        None => {
            println!("No candidates found.");
        },
    }
}
