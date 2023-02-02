fn match_triple(triple: (i32, i32, i32)) {
    match triple {
        (1, 2, 3) => println!("Got 1, 2, 3"),
        (_, 2, 3) => println!("Ends in 2 and 3"),
        (42, _, 42) => println!("Meaning of life"),
        (199, 128, _) => println!("CS 128 Honors!"),
        (128, ..) => println!("We only care that the first item is 128"),
        (.., 2002) => println!("We only care that the last item is 2002"),  
        (x, 1, 1) => println!("got {} and two 1s", x),
        (x, y, z) => println!("triple adds to {}", x + y + z)
    }
}

fn main() {
    let my_triples = [
        (1, 2, 3),
        (10, 20, 30),
        (128, 999, 0),
        (199, 128, 20),
        (128, -128, 5),
    ];

    for triple in my_triples {
        match_triple(triple);
    }
}