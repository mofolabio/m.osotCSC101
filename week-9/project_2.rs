use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
}

fn main() -> std::io::Result<()> {
    let students = vec![
        Student {
            name: "Aisha Ibrahim".to_string(),
            matric_number: "PAU/2020/1234".to_string(),
            department: "Computer Science".to_string(),
        },
        Student {
            name: "Bello Usman".to_string(),
            matric_number: "PAU/2021/5678".to_string(),
            department: "Economics".to_string(),
        },
        Student {
            name: "Chioma Okoro".to_string(),
            matric_number: "PAU/2022/9012".to_string(),
            department: "Law".to_string(),
        },
    ];

    let mut file = File::create("student_details.txt")?;

    for student in &students {
        let student_info = format!(
            "Name: {}\nMatric Number: {}\nDepartment: {}\n\n",
            student.name, student.matric_number, student.department
        );
   
        file.write_all(student_info.as_bytes())?;
        print!("{}", student_info);
    }

    Ok(())
}