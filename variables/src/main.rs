
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
enum IpAddrKind {
    V4 = 100,
    V6 = 200,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}",spaces);

    let guess: u32 = "42".parse().expect("Not a number!");   
    println!("{}",guess);

    let t = true;
    let f: bool = false;
    println!("{},{}",t,f);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is : {},y is: {}, z is :{}",x, y,z);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("{},{}",first,second);

    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    println!("The five function return of number is: {}", five());
    println!("The plus_one function return of number is: {}", plus_one(10));

    let xx : i32 = 12;
    println!("The plus_one function return of number is: {}", plus_one(xx));

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);


    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // let s3 = s1;  // できない
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s = String::from("hello");  
    takes_ownership(s);
    // println!("{}", s); // s move scope
    let x = 5;
    makes_copy(x);  
    println!("{}", x);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s;
    let r1 = &mut s;

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}{}",hello,world);


    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{} {} {} {}",user1.username,user1.email,user1.sign_in_count,user1.active);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(128, 1, 2);
    let origin = Point(255, 3, 4);
    println!("{}",black.0);
    println!("{}",origin.0);

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let sq = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixels.",
        sq.area()
    );

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{}",four as usize);

    let dime = Coin::Dime;
    println!("{}",value_in_cents(dime));
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x); 
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} 

fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} 

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}