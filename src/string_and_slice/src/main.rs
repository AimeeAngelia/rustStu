fn main() {
    let data = "initial contents";
    let s = data.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");
    // let mut s = String::new();
    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("{s5}");
    let mut s6 = String::from("lo");
    s6.push('l');
    println!("{s6}");
    let s7 = s.clone() + "-" + &s2 + "-" + &s3;
    println!("{s7}");
    let s8 = format!("{s}-{s2}-{s3}");
    println!("{s8}");
    let fancharacter = "Здравствуйте";
    println!("{fancharacter}");
    for i in fancharacter.bytes() {
        println!("{i}");
    }
    let fans = &fancharacter[0..2];
    println!("{fans}");
}
