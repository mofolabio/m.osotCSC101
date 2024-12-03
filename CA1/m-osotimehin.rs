use std::io;


struct Sibling {
    first_name: String,
    age: i32,
    gender: String,
    country_of_residence: String,
    marital_status: String,
    children: Vec<String>,  
    city_of_residence: String,
}

fn main() {
    println!("Enter the number of siblings:");

    
    let mut number_of_siblings = String::new();
    io::stdin().read_line(&mut number_of_siblings).unwrap();
    let number_of_siblings: i32 = number_of_siblings.trim().parse().unwrap();

    
    for i in 0..number_of_siblings {
        println!("\nEnter details for sibling {}:", i + 1);

        let mut first_name = String::new();
        println!("Enter first name:");
        io::stdin().read_line(&mut first_name).unwrap();
        let first_name = first_name.trim().to_string();

        let mut age = String::new();
        println!("Enter age:");
        io::stdin().read_line(&mut age).unwrap();
        let age: i32 = age.trim().parse().unwrap();

        let mut gender = String::new();
        println!("Enter gender:");
        io::stdin().read_line(&mut gender).unwrap();
        let gender = gender.trim().to_string();

        let mut country_of_residence = String::new();
        println!("Enter country of residence:");
        io::stdin().read_line(&mut country_of_residence).unwrap();
        let country_of_residence = country_of_residence.trim().to_string();

        let mut city_of_residence = String::new();
        println!("Enter the city where you currently live:");
        io::stdin().read_line(&mut city_of_residence).unwrap();
        let city_of_residence = city_of_residence.trim().to_string();

        let mut marital_status = String::new();
        println!("Enter marital status (married/single/in relationship):");
        io::stdin().read_line(&mut marital_status).unwrap();
        let marital_status = marital_status.trim().to_string();

        let mut children = Vec::new();
        (if marital_status == "married" {
       	println!("Do you have children? (yes/no):");
        let mut has_children = String::new();
        io::stdin().read_line(&mut has_children).unwrap();
        if has_children.trim().to_lowercase() == "yes" {
         
         println!("Enter names of children separated by commas (e.g., Alice, Bob):");
        	let mut children_input = String::new();
         io::stdin().read_line(&mut children_input).unwrap();
                children = children_input.trim().split(',').map(|s| s.trim().to_string()).collect();
            }
        });

        
        let sibling = Sibling {
            first_name,
            age,
            gender,
            country_of_residence,
            marital_status,
            children,
            city_of_residence,
        };

        
        println!("\n--- Sibling Information ---");
        println!("Name: {}", sibling.first_name);
        println!("Age: {}", sibling.age);
        println!("Gender: {}", sibling.gender);
        println!("Country of Residence: {}", sibling.country_of_residence);
        println!("City of Residence: {}", sibling.city_of_residence);
        println!("Marital Status: {}", sibling.marital_status);
        
        if !sibling.children.is_empty() {
            println!("Children: {}", sibling.children.join(", "));
        }

     
        if sibling.age < 18 {
            let mut waec_completed = String::new();
            println!("Have you completed the West African Examinations Council (WAEC) exams? (yes/no):");
            io::stdin().read_line(&mut waec_completed).unwrap();

            if waec_completed.trim().to_lowercase() == "yes" {
                let mut school_name = String::new();
                println!("Enter the name of the secondary school you attended:");
                io::stdin().read_line(&mut school_name).unwrap();

                let mut final_grade = String::new();
                println!("Enter your final grade:");
                io::stdin().read_line(&mut final_grade).unwrap();

                let mut waec_year = String::new();
                println!("Enter the year you completed WAEC:");
                io::stdin().read_line(&mut waec_year).unwrap();

                println!("WAEC Details: {} | {} | {}", school_name.trim(), final_grade.trim(), waec_year.trim());
            } else {
                let mut class_level = String::new();
                println!("Enter your current class level:");
                io::stdin().read_line(&mut class_level).unwrap();

                let mut plan_waec = String::new();
                println!("Do you plan to take WAEC soon? (yes/no):");
                io::stdin().read_line(&mut plan_waec).unwrap();

                if plan_waec.trim().to_lowercase() == "yes" {
                    let mut planned_year = String::new();
                    println!("In what year do you plan to take WAEC?");
                io::stdin().read_line(&mut planned_year).unwrap();
                println!("Planned WAEC Year: {}", planned_year.trim());
                } else {
                    println!("No plans for WAEC.");
                }
            }
        }
    }
}
