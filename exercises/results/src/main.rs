fn main() {
    // when plain    

    let numstr = "6";
    let num = numstr.parse::<i32>();
    let num = num.expect("should have a number");
    println!("num + 5 = {}", num + 5);

    // using result

    let numstr = "florp";
    let num = numstr.parse::<i32>();
    let answer = match num {
        Ok(n) => n + 5,
        Err(_) => 0,
    };
    println!("Answer is {}", answer);

}
