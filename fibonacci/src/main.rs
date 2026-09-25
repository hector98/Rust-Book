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

        let mut ant: u64 = 0;
        let mut res: u64 = 1;
        let mut i: u32 = 0;

        while res <= number{
            print!("{ant}, ");
            let aux: u64 = res;
            res = ant + res;
            ant = aux;
            i += 1;
        }
        print!("{ant}... {i} iteractions\n")
    }
}
