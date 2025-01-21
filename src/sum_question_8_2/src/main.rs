fn main() {
    let text = String::from("first apple");
    let mut vecs = Vec::new();
    for i in text.split_whitespace() {
        let mut count = 0;
        let mut strs = i.to_string();
        // let is_a = false;
        let first_char = strs.chars().next(); // 获取第一个字符
        let is_a = match first_char {
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => true,
            _ => false,
        };
        for m in i.chars() {
            match m {
                'a' => (),
                'e' => (),
                'i' => (),
                'o' => (),
                'u' => (),
                other => {
                    strs.remove(count);
                    strs.push('-');
                    strs.push(other);
                    strs.push_str("ay");
                    if is_a {
                        strs.push_str("-hay");
                    }
                    vecs.push(strs);
                    // println!("{strs}");
                    break;
                }
            }
            count += 1;
        }
    }

    println!("{vecs:?}");
}
