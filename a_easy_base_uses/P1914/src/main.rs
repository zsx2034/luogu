fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: u8 = (buf.trim().parse::<u32>().unwrap() % 26) as u8;

    let base = b'a';
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().char_indices().for_each(|(i, c)| {
        let c = c as u8;
        let d = (c - base + n) % 26 + base;
        print!("{}", d as char);
    });
}
