fn main () {
    let mut s = String::from("hello");
    println!("{s}");
    let s2 = &mut s;
    change(&mut s);
    println!("{s}");
}

fn change(s1: &mut String) {
    s1.push_str(",world!");
}
