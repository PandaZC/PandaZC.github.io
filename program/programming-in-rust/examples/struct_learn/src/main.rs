// struct User {
//     name: String,
//     age: u32,
//     sex: String,
//     email: String,
// }

// fn build_user(name: String, email: String) -> User {
//     User {
//         name,
//         age: 18,
//         sex: String::from("male"),
//         email,
//     }
// }

// fn main() {
//     let user1 = User {
//         name: String::from("Zhang San"),
//         age: 28,
//         sex: String::from("male"),
//         email: String::from("zhangsan123@163.com"),
//     };

//     println!("{} {} {} {}", user1.name, user1.age, user1.sex, user1.email);

//     // struct更新语法
//     let user2 = User {
//         name: String::from("Li Si"),
//         email: String::from("lisi456@163.com"),
//         ..user1
//     };
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn cal_rectangle_area(&self) -> u32 {
        self.width * self.height
    }

    fn check_hold_another(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn create_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 10,
        height: 50,
    };

    let rectangle1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("{} {}", rectangle.check_hold_another(&rectangle1), rectangle.check_hold_another(&rectangle2));

    let square = Rectangle::create_square(5);
    println!("{}", square.cal_rectangle_area());
}

// #[derive(Debug)]
// struct Color(i32, i32, i32);
// struct Coordinate(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let point = Coordinate(0, 0, 0);
//     println!("{:?}", black);
//     println!("{:?}", point);
// }