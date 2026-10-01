//-----> Creacion de diferentes tipos con estrutura de tupla <--- 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//---> Definicion de la estructura tipo unidad <----------- 
struct AlwaysEqual;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//---> Ejemplo area de rectangulo <------------- 
#[derive(Debug)] //Atributo 'outer'
struct Rectangle {
    width: u32,
    height: u32,
}

//------> Metodos <-------------------------- 
impl Rectangle { // Bloque de implementacion para Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //-----> Funciones asociadas <----------------------- 
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

//----> Metodos con mas parametros <---------------------- 


fn main() {
    //--------->Uso de la estructura 'User'<------- 
    let user1 = User {
        active: true,
        username: String::from("barrios98"),
        email: String::from("barrioshector13@gmail.com"),
        sign_in_count: 1,
    };

    //---------------> Usar instancia mutable para editar<------------ 
    let mut user1 = User {
        active: true,
        username: String::from("barrios98"),
        email: String::from("othermail"),
        sign_in_count: 1,
    };

    //---->Uso de . para acceder a un elemento
    user1.email = String::from("barrioshector13@gmail.com");

    //---> Creacion de instancias con la sintaxis de actualizacion de estructura
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    //------> Tuplas <-------------------- 
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //----------> Estructura tipo Unitaria <--------------- 
    let subject = AlwaysEqual;

    //-------> Ejemplo area de un Rectangulo <---------------- 
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        );

    //-------> Agregar funcionalidad con rasgos derivados <---- 
    println!("rect1 is {rect1:?}");
    // Or 
    println!("rect1 is {rect1:#?}");

    //-----> dbg! <--------------------- 
    dbg!(&rect1);


    //--------------------------------------------------------- 
    //--------> Metodos <-------------------------------------- 
    //----------------------------------------------------------
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The are of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() { // width() llama al metodo
        // width sin parentesis hace referecia al campo
        println!("The rectangle has nonzero width; it is {}", rect1.width);
    }


    //----> Metodos con mas parametros
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    //----> Uso de funcion asociada <-------------------- 
    let sq = Rectangle::square(3);
    //println!("Rectangle square: {}", sq);
}

//-------> Ejmeplo area de un rectangulo <------- 
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//---> Uso de la notacion abreviada de inilizacion de camoo
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
