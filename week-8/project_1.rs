#[derive(Debug)]
struct StaffLevel {
    role: String,
    experience: String,
    aps_level: String,
}

fn main() {
    let staff_levels = vec![
        StaffLevel {
            role: String::from("Office Administrator"),
            experience: String::from("1-2"),
            aps_level: String::from("APS 1-2"),
        },
         StaffLevel {
            role: String::from("Office Administrator"),
            experience: String::from("3-5"),
            aps_level: String::from("APS 3-5"),
        },
         StaffLevel {
            role: String::from("Office Administrator"),
            experience: String::from("5-8"),
            aps_level: String::from("APS 5-8"),
        },
         StaffLevel {
            role: String::from("Office Administrator"),
            experience: String::from("8-10"),
            aps_level: String::from("EL1 8-10"),
        },
       StaffLevel {
            role: String::from("Academic Lawyer"),
            experience: String::from("1-2"),
            aps_level: String::from("APS 1-2"),
        },
         StaffLevel {
            role: String::from("Academic Lawyer"),
            experience: String::from("3-5"),
            aps_level: String::from("APS 3-5"),
        },
          StaffLevel {
            role: String::from("Academic Lawyer"),
            experience: String::from("5-8"),
            aps_level: String::from("APS 5-8"),
        },
          StaffLevel {
            role: String::from("Academic Lawyer"),
            experience: String::from("8-10"),
            aps_level: String::from("EL1 8-10"),
        },
         StaffLevel {
            role: String::from("Teacher"),
            experience: String::from("1-2"),
            aps_level: String::from("APS 1-2"),
        },
          StaffLevel {
            role: String::from("Teacher"),
            experience: String::from("3-5"),
            aps_level: String::from("APS 3-5"),
        },
          StaffLevel {
            role: String::from("Teacher"),
            experience: String::from("5-8"),
            aps_level: String::from("APS 5-8"),
        },
         StaffLevel {
            role: String::from("Teacher"),
            experience: String::from("8-10"),
            aps_level: String::from("EL1 8-10"),
        },
    ];


    let input_role = String::from("Academic Lawyer");
    let input_experience = String::from("5-8");

    for staff_level in &staff_levels {
        if staff_level.role == input_role && staff_level.experience == input_experience {
           println!("The APS Level for {} with {} years of experience is: {}", input_role, input_experience, staff_level.aps_level );
        }
    }
}