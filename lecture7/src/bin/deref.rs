fn main() {
    let mut cs128h: String = "CS 128 Honors".to_string();
    let mut num = 128; 

    hello(&mut cs128h);

    println!("{}", cs128h);

    greet(&mut cs128h);

    println!("{}", cs128h);

    increment(&mut num);
    println!("num={}", num);
    increment(&mut num);
    println!("num={}", num);
}

fn hello(s: &mut String) {
    *s = "Hello ".to_string() + s + "!";
}

fn greet(s: &mut String) {
    s.push_str(" It is nice to meet you!");
}

fn increment(x: &mut i32) {
    *x = *x + 1;
}