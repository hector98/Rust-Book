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
    println!("The largest ");

    //--> Uso de Rasgos <----- 
    let post = SocialPost {
        username: String::from("Horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
    };

    println!("1 new post: {}", post.summarize());


    //-------------------------------------------------- 
    //--> Validacion de referencias con tiempo de vida <-- 
    //---------------------------------------------------- 

    //----> Referencias colgantes <------------- 
    //

    //-----> Duracion generica de las funciones <--- 
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest(x: &str, y: &str) -> &str { // Error
    if x.len() > y.len() { x } else { y }
}

//---> Sintaxis de tiempo de vida <------ 
&i32         // a reference
&'a i32      // a reference with an explicit lifetime
&'a muit i32 // a mutable reference with an explicit liftime

//--> En las firmas de funciones <---------- 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

//--> En definicion de Estructuras <---------- 
struct ImportantExcerpt<'a> {
    part: &'a str,
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


//--> Definir el comportamiento compartido mediante rasgos <-- 
//--> Un rasgo es similar a usar interfaces <----- 

//--> Definir Rasgo <------- 
pub trait Summary {
    fn summarize(&self) -> String;
}

//--> Implementacion de rasgo en un tipo <--- 
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//--> Utilizar rasgos como parametros <----- 
pub fn notify(item: &impl summary) {
    println!("Breaking news! {}", item.summarize());
}

//--> Sintaxis ligada a rasgos <------- 
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

//--> Multiples limites de ragos con sintaxis + <--- 
pub fn notify(item:: &(impl Summary + Display)) {}

pub fn notify<T: Summary + Display>(item: &T){}

//----> Limites de ragos mas claros con clasula where <--- 
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

//----> Tipos de retorno que implementan rasgos <--- 
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
    }

}

//---> Uso limi


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
