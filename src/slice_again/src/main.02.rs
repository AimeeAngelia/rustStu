fn main() {
    let s = String::from("hello, world!");

    let slice = &s[..2];
    println!("{s}");
    println!("{slice}");

    let slice2 = &s[5..];
    println!("{slice2}");
    let slice3 = &s[..];
    println!("{slice3}");
}
