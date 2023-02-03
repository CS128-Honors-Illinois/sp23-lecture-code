fn get_prof(course: i32) -> Option<String> {
    let professor: Option<String> = match course {
        124 => Some("Prof. Challen".to_string()),
        128 => Some("Prof. Nowak".to_string()),
        173 => Some("Prof Fleck".to_string()),
        225 => Some("Prof Evans".to_string()),
        _ => None
    };

    return professor;
}

fn main() {
    let courses = [124, 128, 173, 225, 361];

    for course in courses {
        let professor = get_prof(course);

        match professor {
            Some(name) => println!("{} teaches CS {}", name, course),
            None => println!("It is unclear who teaches CS {}", course)
        }
    
        // if professor.is_none() {
        //     println!("It is unclear who teaches CS {}", course);
        // } 

        // if professor.is_some() {
        //     let name = professor.unwrap(); // be careful
        //     println!("{} teaches CS {}", name, course);
        // }

        println!("-----------")
    }
}