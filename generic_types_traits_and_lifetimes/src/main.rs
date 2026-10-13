//----> En definicion de estructuras <----- 
struct Point<T> {
    x: T,
    y: T,
}

//----> Usando dos genericos <------ 
struct Point_2<T, U> {
    x: T,
    y: U,
}

//---> En definicion de enumeraciones <---- 
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

//-----> En definicion de metodos <----- 
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


fn main() {
    //---> en definicion de metodos <---- 
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    //---> Genericos en estructuras <------- 
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    //----> Usando dos tipos genericos <---- 
    let both_integer = Point_2 { x: 5, y: 10 };
    let both_float = Point_2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point_2 { x: 5, y: 4.0};


    //----> Elimincion de duplicados mediante
    //-- la extraccion de una funcion <------- 
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest numner is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    //--> usando la abstraccion largest <---- 
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_func(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_func(&number_list);
    println!("The largest number is {result}");


    //--------------------------------------------- 
    //--> Tipos de datos genericos <--------------- 
    //--------------------------------------------------- 
    let number_list = vec![32, 54, 53, 67, 88, 99];
    
    let result = largest_generic(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['h', 'e', 'l', 'l', 'o'];
    
    let resutl = largest_generic(&char_list);
    println!("The largest ")
}


//--> usando una abstraccion para encontrar el numero mas grade <-- 
fn largest_func(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


//--> En  definicion de funciones <-------------------- 
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = &list[0];

    for item  in list {
        if item > largest {
            largest = item;
        }
    }

    *largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item  in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item  in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
