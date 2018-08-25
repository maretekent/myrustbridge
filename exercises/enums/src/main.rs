 #[warn(dead_code)]
 #[warn(unused_variables)] 
 
fn main() {
    enum TrafficLight {
        Red,
        Yellow,
        Green,
        blue,
    }
    let light = TrafficLight::Green;
    let color = TrafficLight::blue;
    match color {
        TrafficLight::Red => println!("STOP!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go go go!"),
        _ => println!("enum not found"),
    }
}
