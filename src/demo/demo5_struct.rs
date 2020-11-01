pub fn run() {
    // 结构体的实例
    let mut user = User {
        username: String::from("jojo"),
        email: String::from("feafaw@email.com"),
        age: 12,
        active: true,
    };
    // 你可以修改可变类型的结构体的字段
    user.age = 24;

    let user2 = builder_user(String::from("tony"), String::from("tony@email.com"), 12);

    let user3 = User { age: 15, ..user2 }; // 可从user2中获取未设字段

    let black = Color(0, 0, 0); // 元组结构体使用方法也和元组一样
    let r = black.0;

    let rectangle = Rectangle {
        width: 15,
        height: 20,
    };

    // 函数参数
    area(&rectangle);
    // 使用注解就可输出结构体
    println!("rectangle:{:?}", rectangle);
    println!("结构输出 rectangle:{:#?}", rectangle);

    // rectangle 根据 area方法,自动引用和解引用
    // 因为 area 方法中明确规定了 实例的使用方式 &self ; &mut self ;self
    rectangle.area();
    // 使用关联函数创建实例
    let rectangle2 = Rectangle::factory(12, 50);
}

fn builder_user(username: String, email: String, age: u8) -> User {
    User {
        username, // 简写
        email,
        age: age,
        active: true,
    }
}

// 使用注解,派生Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl(implementation) 结构体的方法
impl Rectangle {
    // 使用引用,一般不会在方法中获取实例的所有权
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 结构体中不引用实例的函数被称为为关联函数(静态方法)
    fn factory(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 结构体的定义
struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}

// 元组结构体的定义
struct Color(i32, i32, i32);
