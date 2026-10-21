use std::env;

fn main() {
    //--> Lectura de los valores del argumento <-- 
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    //----> Guardar los valores de los argumentos en variables <----- 
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
