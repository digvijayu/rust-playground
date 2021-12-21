struct User {
    name: String,
    age: u8,
    email: String
}

fn main() {
    let user1 = User {
        name: String::from("player1"),
        age: 12,
        email: String::from("A@B.C")
    };

    let user2 = User {
        name: String::from("some name"),
        ..user1
    };

    println!("name user 2 {}", user2.name); // name user 2 some name
    println!("name user 1 {}", user1.name); // name user 1 player1

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    
}
