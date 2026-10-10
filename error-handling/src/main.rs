use std::fs::File;
use std::io::ErrorKind;

//---> Tipos personalizados para validacion <--------- 
// En base al juego del capitulo 2 
// validar si el numero esta dentro del rango 1..100
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

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

    //-------> Atajos para el panica en caso de error <----- 
    let greeting_file = File::open("hello.txt").unwrap();

    //---> Con expect <------------------------------ 
    let greeting_file = File::opem("hello.txt")
        .expect("hello.txt should be include in this project");

    //-------------> Propagacion de errores <--------
}


//---> Operdador ? <---------------------- 
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
