fn main() {
    let mut i = 1;
    while i <= 15 {
        let mut f = "";
        let mut b = "";
        if i % 3 == 0 {
            f = "fizz";
        }
        if i % 5 == 0 {
            b = "buzz";
        }
        println!("{}{}{}",i,f,b);
        i+=1;
    }
}
