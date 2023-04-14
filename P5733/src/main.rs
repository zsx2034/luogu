fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    for ch in buf.chars() {
        if ch.is_ascii_alphabetic() {
            print!("{}", ch.to_ascii_uppercase());
        } else {
            print!("{}", ch);
        }
    }
}
