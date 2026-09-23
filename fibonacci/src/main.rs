use std::io;

fn main() {
    println!("Welcome!!!!");
    
    loop {
        println!("Please write a positive number!!");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line()");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        println!("Your number is: {number}");
    }
}
