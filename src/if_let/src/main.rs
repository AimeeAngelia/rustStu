#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    // let config_max: Option<u8> = None;
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!")
    } else {
        count += 1;
        println!("{count}");
    }
}
