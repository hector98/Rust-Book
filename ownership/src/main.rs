fn main() {
    let _s = "hello";

    // Cadena mutada
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String 

    println!("{s}");

    // Varibales e interaccion de datos
    let x = 5;
    let y = x;

    // String version str 
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); Error

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world");

    // Variables y datos que 
    // interactuan con Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Datos solo de pila: Copiar
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    
    //////////////////////////_//
    // Propiedad y Funciones
    ///////////////////////////
    let s = String::from("hello"); // s comes into scope 

    takes_ownership(s);            // s's value moves into the function...
                                   // ... and so is no longer valid here 

    let x = 5;                     // x comes into scope 

    makes_copy(x);                 // Because i32 implements the Copy trait,
                                   // x does NOT move into the function,
                                   // so it's okay to use x afterward


    //////////////////////////////////////////////
    // Referencias y prestamos
    /////////////////////////////////////////////
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");


    //////////////////////////////////////////////////
    // Referencias mutables
    /////////////////////////////////////////////////
    let mut s = String::from("Hello");

    change(&mut s);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Varibales r1 and r2 will not be used after this point.
    
    let r3 = &mut s; // No problem
    println!("{r3}");


    /////////////////////////////////////////////////////
    // Referencias colgantes
    ////////////////////////////////////////////////////
    // No existen en Rust


    /*---------------------------------------------------------- 
     -------> El Tipo Rebabada <---------------------- 
    ---------------------------------------------------------*/ 
    //----------> Rebanada de Strings <-------------------- 
    let s = String::from("hello wolrd");

    let hello = &s[0..5];
    let wolrd = &s[6..11];

    //------------------> Literales de String como segmentos <------- 
    let s = "Hello, world!";

    let my_string = String::from("Hello world");

    // 'first_word' works on slices of 'String's, wheter partial or whole 
    let word = first_word)(&my_string[0..6]);
    let word = first_word(&my_string[..]);


    //----------> Otras Renamadas (Slices) <------------- 
    let a = [1,2,3,4,5,6];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);


} // Here, x goes out of scope, then s. However, because s's value was moved,
  // Nothing special appens 

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convierte un string en un array de caracteres

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn takes_ownership(some_string: String) { // Some_string comes into scope

    println!("{some_string}");
} // Here, some_string goes out of scope and 'drop' is called. The backing
  // memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here,some_integer goes out of scope. Nothing special happens.
