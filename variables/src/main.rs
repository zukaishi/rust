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
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x); 
}
