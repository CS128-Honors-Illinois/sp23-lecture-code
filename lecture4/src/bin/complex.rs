fn match_course(course: u32) -> String {
    let msg: String = match course {
        0 ..= 99 => "INVALID NUMBER".to_string(),
        128 | 225 | 341 => "Teaches C or C++".to_string(),
        100 ..= 199 => "100 Level".to_string(),
        level @ 100 ..= 399 => {
            let hundreds_digit: u32 = level / 100;
            format!("{}00 level course", hundreds_digit)
        },
        num @ 400 ..= 499 => if num == 461 {
                "My favorite class".to_string()
            } else {
                "Upper level electives".to_string()
            },
        500 ..= 599 => "Graduate level".to_string(),
        n => format!("CS {} is not a value course!", n)
    };

    return msg;
}

fn main() {
    let course_nums: [u32; 10] = [124, 128, 233, 361, 225, 341, 440, 10, 523, 699];

    for course in course_nums {
        let msg = match_course(course);
        println!("CS {} - {}", course, msg);
    }
}