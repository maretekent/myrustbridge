#[warn(dead_code)]

fn main() {
    enum GameType {
        SinglePlayer,
        MultiPlayer(u32),
    }
    let game = GameType::MultiPlayer(4);
    match game {
        GameType::SinglePlayer => println!("How about solitaire?"),
        GameType::MultiPlayer(2) => println!("How about checkers?"),
        GameType::MultiPlayer(4) => println!("How about bridge?"),
        GameType::MultiPlayer(num) => {
            println!("How about {}-player tag?", num)
        },
    }
}
