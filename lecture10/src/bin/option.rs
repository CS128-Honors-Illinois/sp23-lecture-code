#[derive(Debug)]
enum MyOption<S> {
    Some(S),
    None
}

impl<S> MyOption<S> {
    fn unwrap(&self) -> &S {
        match self {
            MyOption::Some(x) => &x,
            MyOption::None => panic!("unwrap on a None")
        }
    }

    fn is_some(&self) -> bool {
        match self {
            MyOption::Some(_) => true,
            MyOption::None => false
        }
    }

    fn is_none(&self) -> bool {
        !self.is_some()
    }
}

fn main() {
    let _ = Some(5);
    let opt: MyOption<String> = MyOption::Some("CS 128 Honors".to_string());

    if opt.is_some() {
        println!("Found value in MyOption: {}", opt.unwrap());
    }

    let empty: MyOption<u8> = MyOption::None;

    if empty.is_none() {
        println!("Found empty MyOption");
    }
    empty.unwrap(); // WARNING: THIS WILL PANIC (!!!)
}