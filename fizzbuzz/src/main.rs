fn main() {
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
