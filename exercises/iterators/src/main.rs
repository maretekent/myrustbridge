fn main() {
    for i in (0..10).filter(|x| x % 2 == 0) {
        println!("i = {}", i);
    }
    for i in (0..10).map(|x| x * x ) {
        println!("i = {}", i);
    }
    let sum = (0..10).fold(0, |acc, x| acc + x);
    println!("sum = {}", sum);
}
