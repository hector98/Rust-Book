use std::io;

fn main() {
    println!("Hello, Welcome to conversor of Fahrenheit to Celsius");

    loop {
        println!("\t\t>>>> Menu <<<<
        (1)-> Fahrenheit to Celsius.
        (2)-> Celsius to Fahrenheit.
        Plesese write 1 or 2 🙃");

        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read line()");

        let op: u8 = match op.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if op == 1 {
            let f_t_c: f64 = fahrenheit_to_celsius();

            println!("{f_t_c}");
        }
    }
}

fn fahrenheit_to_celsius () -> f64 {
    println!("Please write the Fahrenheit grados!");

    let  mut grados = String::new();
    io::stdin()
        .read_line(&mut grados)
        .expect("Failed to read line()");

    let grados: f64 = match grados.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let celsius :f64 = (grados - 32.0) / 1.8

    return grados;
}
