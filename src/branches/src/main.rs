fn main() {
    let number = 3;

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let condtions = true;
    let x: i32 = if condtions {5} else {"6"};

    println!("The value of x is {x}");
}
