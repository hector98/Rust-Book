//--> Proporcionar nuevos nombre con la palabra clave as 
use std::fmt::Result;
use std::io::Result as IoResult;

//--> Uso de rutas anidadas
use std::{cmp::Ordering, io};
use std::io::{self, Write};

//---> Importacion de articulos con el operador Glob 
use std::collections::*;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult {
    // --snip--
}

mod front_of_house {
    // Usando la palabra clave pub
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

//----> Palabra clave use 
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path 
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the sumer with Rye toast 
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about waht bread we'd like. 
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

//------> Iniciando Rutas relativas con Super <----- 
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    //--> Hacer publicas las estructuras y enums
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
