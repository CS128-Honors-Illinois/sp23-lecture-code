fn main() {
    let class: String = "CS 128 Honors".to_string();

    say_hello_borrow(&class);

    say_hello(class);
}

fn say_hello(name: String) { 
    // say_hello takes ownership of `name` here
    println!("Hello {}!", name);
} // name is dropped here

fn say_hello_borrow(name: &String) { 
    // say_hello gets a reference to a string here
    println!("Hello {}!", name);
} // the original String remains after this function
