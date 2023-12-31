struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {

    let mut user1 = User {
        email: String::from("lee@gmail.com"),
        username: String::from("lee"),
        active: true,
        sign_in_count: 1,
    };


    let name = user1.username;
    user1.username = String::from("lee2");

    let user2 = build_user(
        String::from("lee123@gmail.com"), 
        String::from("lee123")
    );

    let user3 = User {
        email: String::from("james@gmail.com"),
        username: String::from("james"),
        ..user2
    };

    // Tuple Structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 55
    };

    let rect4 = Rectangle::square(30);

    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("rect can hold rect3: {}", rect.can_hold(&rect3));
    println!("rect can hold rect4: {}", rect.can_hold(&rect4));

    println!("rect: {:#?}", rect);

    println!("The area of the rectangle is {} square pixels.", rect.area());

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}

