fn main() {
    // vactors are dynamic arrays
    let mut prices = vec![30, 100, 2];
    prices[0] = 25;
    prices.push(40);
    println!("All the prices are: {:?}", prices);
}
