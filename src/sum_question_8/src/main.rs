use std::collections::HashMap;

fn main() {
    let mut nums = vec![3, 5, 2, 7, 3, 9, 10, 43, 56, 27, 6, 8];
    nums.sort();
    println!("{nums:?}");
    let nums_len = nums.len();
    let nums_middle = nums[nums_len / 2 - 1];
    println!("中位数是: {nums_middle}");
    let mut map = HashMap::new();
    for i in &nums {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    // println!("{map:#?}");
    let max_entry = map.iter().max_by_key(|&(_, &value)| value);
    match max_entry {
        Some((k, v)) => println!("众数是{k}, 出现次数: {v}"),
        _ => (),
    }
}
