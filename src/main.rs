use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    println!("Welcome to Dzi lisp");
    println!("To exit type Ctrl + c");
    
    loop {
        print!("dzilisp> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        println!("You've typed {}", input);
        input.clear();
    }
}
