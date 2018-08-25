fn main() {
    //This prints hello world
    // This is a comment
    println!("Hello, world!");
    let mut x = 1;

    while x < 10 {
        if x == 6 { continue;}
        println!(" x is {}", x);
        x += 1;
    }
}
