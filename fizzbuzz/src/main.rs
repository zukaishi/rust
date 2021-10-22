fn main() {
    fizzbuzz1();
    fizzbuzz2();
}
fn fizzbuzz1(){
    println!("### fizzbuzz1 ###");
    for i in 1..16 {
        let mut s: String = "".to_string();
        if i % 3 == 0 {
            s.push_str("fizz");
        }
        if i % 5 == 0 {
            s.push_str("buzz");
        }
        println!("{}{}",i,s);
    }
}
fn fizzbuzz2(){
    println!("### fizzbuzz2 ###");
    for i in 1..16 {
        match i {
            a if a % 15 == 0 => println!("FizzBuzz"),
            a if a % 3 == 0 => println!("Fizz"),
            a if a % 5 == 0 => println!("Buzz"),
            a => println!("{}", a),
        }
    }
}