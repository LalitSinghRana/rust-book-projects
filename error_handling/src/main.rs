// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

// use std::fs::File;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");
// }


// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap();
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}




