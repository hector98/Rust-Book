fn main() {
    //------------------------------------------ 
    //--> Errores irrecuperables con panic! 
    //------------------------------------------- 

    //--> LLamar a panic! <--- 
    panic!("crash and burn");

    //--> Provocar panic con indice invalido <--- 
    let v = vec![1, 2, 3];
    v[99]; //Aqui se llama a panic!
}
