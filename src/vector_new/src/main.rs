use std::any::type_name;

fn print_type<T>(_:T) {
    println!("The type is: {}", type_name::<T>());
}

fn main() {
    let mut v = Vec::new();
    let v1 = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let thirds: Option<&i32> = v.get(2);
    match thirds {
        Some(thirds) => println!("The third element is {thirds}"),
        None => println!("There is no third element!"),
    }

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
        print_type(i);
        // println!(i);
    }

    enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
for i in &row {
match i {
    SpreadsheetCell::Int(Intnum) => {
        print_type(Intnum);
        println!("{Intnum}");
    },
    
    _ => (),
}
}
}
