fn main() {
    //--------->Uso de la estructura 'User'<------- 
    let user1 = User {
        active: true,
        username: String::from("barrios98"),
        email: String::from("barrioshector13@gmail.com"),
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
