use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //------------------------------------------ 
    //--> Errores irrecuperables con panic! 
    //------------------------------------------- 

    //--> LLamar a panic! <--- 
    //panic!("crash and burn");

    //--> Provocar panic con indice invalido <--- 
    let v = vec![1, 2, 3];
    //v[99]; //Aqui se llama a panic!



    //---> Errores recuperables con Result <-- 
    // Intentar abrir un archivo no existente
    let greeting_file_result = File::open("Hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };


    //---> Coincidencia basada en diferentes errores <------ 
    let greeting_file_result = File::open("Hello.txt");
    let greetimg_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => { // Uso de del comodin _ para cualquier valor
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
