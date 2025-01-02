fn main() {
    let candidates = vec![
        ("Alice", 5),
        ("Bob", 10),
        ("Charlie", 3),
        ("David", 12),
        ("Eve", 8),
    ];

    let mut max_experience = 0;
    let mut best_candidate = "";

    for (name, experience) in candidates {
        if experience > max_experience {
            max_experience = experience;
            best_candidate = name;
        }
    }
    println!("The candidate with the most experience is: {}, with {} years of experience", best_candidate, max_experience);
}