fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut cnt = 0;
    buf.chars().for_each(|c| {
        if c.is_ascii_alphabetic() || c.is_alphanumeric(){
            cnt += 1;
        } 
    });

    println!("{}", cnt);
}
