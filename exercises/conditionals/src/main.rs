fn main() {
    let height = 167u32;
    if height < 150 {
        println!("You're too small to go on the rollercoaster.");
    } else if height < 200 {
        println!("You may go on the rollercoaster!");
    } else {
        println!("You're too tall to go on the rollercoaster.");
    }
}
