fn main() {
    let x = add_five_to_string(String::from("12"));
    println!("Answer");
}


fn add_five_to_string(s: String) ->
    Result<i32, std::num::ParseIntError> {
    let ans = s.parse::<i32>()? + 5;
    Ok(ans)
}
