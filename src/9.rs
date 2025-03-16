use std::fs;

fn main() {
    let mut file = fs::File::create("file.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
