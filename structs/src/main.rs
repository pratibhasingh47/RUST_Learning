struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("pratibha"),
        email: String::from("pratibha@.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("ps@example.com");

    let _name = user1.username;
    user1.username = String::from("Pratibhaa");


    let _user2 = User {
        username: String::from("new_user"),
        email: String::from("another@example.com"),
        active: todo!(),
        sign_in_count: 2,
    };
    
    let _user3 = User {
        username: String::from("new_user_user"),
        email: String::from("another@example2.com"),
        .._user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);


    let width1 = 30;
    let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );



}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // } SAME
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
