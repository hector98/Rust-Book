enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Wrtie(String),
    ChangeColor(i32, i32, i32),
}

//--> Definir metodos para enums <------- 
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

//----> El Option enum <------------- 
enum Option<T> {
    None,
    Some(T),
}

//-------------> La match construction del flujo de control <------- 
//--------> Ejemplo con monedas EEUU <------------------- 
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    //--> Uso de metodos en enums <------- 
    let m = Message::Write(String::from("Hello"));
    m.call();

    //------> Ejemplos de Option valores <-------- 
    let some_number = Some(5);
    let some_char = Some('b');

    let absent_number: Option<i32> = None;

    //-----> El Option<T> matchpatron <----------- 
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //---> Patrones comodin y el _marcador de posicion <-- 
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    //---> Flujo de control conciso con if letylet..,else 
    let config_max = Some (3u8);

    //--> Con match <------------------------------
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    //---> Con if let <----------------- 
    if let Some(max) = config_max {
        println!("The maximun is configured to be {max}");
    }
}


//--> Patrones comodin y el _marcador de posicion
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player() {}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Peny => {
            println!("Lucky penny!");
            1
        } // No es necesario usar coma ',' aqui
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Stste quarter from {stste:?}!");
            25
        }
    }
}

//-----> El Option<T> matchpatron
fn plus_one(x: Option<i32>) Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
