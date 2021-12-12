use std::io;

struct Engineer {
    name: String,
    surname: String,
    job_title: String
}

fn trim_string(s: &str) -> String {
    (&s.clone().trim()).to_string()
}

fn create_engineer() -> Engineer {
    let mut engineer_details: Vec<String> = vec![String::new(); 3];

    println!("Type in name, surname and job title, pressing [Enter] after each");
    for i in 0..engineer_details.len() {
        io::stdin()
            .read_line(&mut engineer_details[i])
            .expect("Error")
            .to_string();
    }

    Engineer {
        name: trim_string(&engineer_details[0]),
        surname: trim_string(&engineer_details[1]),
        job_title: trim_string(&engineer_details[2])
    }
}

fn print_engineer_details(e: &Engineer) {
    println!("{} {}: {}", e.name, e.surname, e.job_title)
}

fn main() {
    let engineer = create_engineer();
    print_engineer_details(&engineer);
}
