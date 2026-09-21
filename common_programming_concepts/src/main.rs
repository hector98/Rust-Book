fn main() {
    //////////////////////////////////////////////////////////
    // Variables y mutabilidad
    //////////////////////////////////////////////////////////
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    /////////////////////////////////////////////////////////
    // Constantes
    ////////////////////////////////////////////////////////
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");


    ///////////////////////////////////////////////////////////////
    // Shadowing (Somnreado)
    //////////////////////////////////////////////////////////////
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    /////////////////////////////////////////////////////////////
    // Data Types 
    ////////////////////////////////////////////////////////////

    // Integer type (u8, u16, u32, u128, usize) 
    let number_int: u32 = "1998".parse().expect("Not a number");
    println!("Integer number {number_int}");

    // Negative integer type (i8, i16, i32, i64, i128, isize)
    let negative_number_int: i8 = "-5".parse().expect("Not a number");
    println!("Negative number {negative_number_int}");

    // Decimal integer
    let number_decimal = 98_222;
    println!("Decimal number {number_decimal}");
    
    // Hexadecimal integer
    let number_hex = 0xff;
    println!("Hexadecimal number: {number_hex}");

    // Octal integer
    let number_octal = 0o77;
    println!("Octal number: {number_octal}");

    // Binario integer
    let number_binario = 0b1111_0000;
    println!("Binary number: {number_binario}");

    //Byte (u8 solamente)
    let number_byte = b'B';
    println!("Byte 'b': {number_byte}");


    /////////////////////////////////////////////////////////////////////////////
    // Float Types
    /////////////////////////////////////////////////////////////////////////////
    let x = 2.0; // f64 
    println!("number float f64: {x}");

    let y: f32 = 3.0; //f32 
    println!("number float f32: {y}");


    /////////////////////////////////////////////////////////////////////////////
    // Numeric operations
    ////////////////////////////////////////////////////////////////////////////
    // additiom
    let sum = 5 + 10;
    println!("Sum 5 + 10 = {sum}");

    // Substraction
    let difference = 95.5 - 4.3;
    println!("Difference 95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product 4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("quetient 56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3;
    println!("truncated -5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder 43 % 5 = {remainder}");


    //////////////////////////////////////////////////////////////
    // Boolean type 
    ////////////////////////////////////////////////////////////
    let t = true;
    println!("Boolean {t}");

    let f: bool = false;
    println!("Boolean {f}");


    ////////////////////////////////////////////////////
    // character type
    //////////////////////////////////////////////////
    let c = 'z';
    println!("Character {c}");

    let z: char = 'ℤ';
    println!("Character {z}");

    let heart_eyed_cat = '😻';
    println!("Heart eyed cat {heart_eyed_cat}");


    ///////////////////////////////////////////////
    // Tupla type 
    //////////////////////////////////////////////
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Obtener valores individuales de la Tupla
    let (x, y, z) = tup;
    println!("Value x = {x}");
    println!("Value y = {y}");
    println!("Value z = {z}");

    // Obtener valores de tupla usando .
    let five_hundred = tup.0;
    println!("tup.0 = {five_hundred}");
    let six_point_four = tup.1;
    println!("tup.1 = {six_point_four}");
    let one = tup.2;
    println!("tup.2 = {one}");


    //////////////////////////////////////////
    // Array type
    /////////////////////////////////////////
    let months = ["January", "February", "March", "April", "May", "June", "July", 
    "August", "September", "Octuber", "November", "December"];

    let a: [i32; 5] = [1,2,3,4,5];

    let a = [3; 5]; // [3,3,3,3,3]

    let first = months[0];
    println!("First month = {first}");

    ///////////////////////////////////////////
    // Functions
    ///////////////////////////////////////////
    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value en function five is: {x}");


    ///////////////////////////////////////////////
    // Control Flow
    ///////////////////////////////////////////////
    
    // if expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // else if 
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if in one line
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Loops
    // loop
    //loop {
     //   println!("again!");
    //}

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop 
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For loop 
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // For loop with range 
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}


fn another_function(x: i32){
    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is {value}{unit_label}");
}

// Function with return
fn five() -> i32 {
    5
}
