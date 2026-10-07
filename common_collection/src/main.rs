use std::collections::HashMap;

fn main() {
    //---> Almacenamiento de listas de valores con verctores <-- 
    // Crear vector vacio 
    let v: Vec<i32> = Vec::new();

    // Vector con valores iniciales
    let v = vec![1, 2, 3];

    //--> Agregar elementos a un vector <---- 
    let mut v = Vec::new();
    v.push(0);
    v.push(2);
    v.push(4);
    v.push(8);

    //---> Lectura de elementos vectoriales <----- 
    let v = vec![1, 2, 3, 4, 5];

    //--> Metodo por indexacion <----
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    //--> Usando el metodo get <----------
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    };

    //---> Iterar sobre los valores de un vector <------ 
    // Usando valores inmutables
    for i in &v {
        println!("{i}");
    }

    //Usando Valores mutables (modificar valores)
    let mut v = [100, 32, 57, 88];
    for i in &mut v {
        *i *= 22;
    }

    //-> Uso de una enum para almacenar multiples tipos <--- 
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Red")),
        SpreadsheetCell::Float(1.12),
    ];

    // uso del metodo pop para eliminar el ultimo elemento

    //----------------------------------------------- 
    //--> Almacenamiento de texto codificado en UTF-8 con cadenas de texto <- 
    //------------------------------------------------- 

    // Creando nuevo String 
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also work on a literal directly:
    let s = "initial contents".to_string();

    // Actualizar un String 
    let mut s = String::from("foo"); // o "foo".to_string
    s.push_str("bar");

    // Concatenar con formato + 
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used 

    // Conctenar con la macro format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //--> Indexacion de Strings 
    let hello = String::from("Hello");
    // let h = hello[0]; --> Error.

    //----> Cortar Cadenas <------------------------ 
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    //----> Iterar sobre Strings <----------------- 
    //Con Char
    for c in "Зд".chars() {
        println!("{c}");
    }

    //Con bytes
    for b in "Зд".bytes(){
        println!("{b}");
    }


    //------------------------------------------------------ 
    //--> Almacenamiento de claves con valores asocidos
    //   en mapas hash <------------------------------- 
    //------------------------------------------------------- 

    // Creacion de un nuevo mapa HASH
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Acceso a valores de un mapa hash (metodo get)
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterar en hash
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Sobrescribir un valor 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    //--> Agregar una clave y valor solo si no hay 
    //una clave presente 
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    //--> Actualizar un valor basandose en el valor anterior.
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
