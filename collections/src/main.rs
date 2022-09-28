fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world hey hello world might world will world hello";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
        // let count = map.entry(word).or_insert(0);
        // *count += 1;
    }

    println!("{:?}", map);
}
