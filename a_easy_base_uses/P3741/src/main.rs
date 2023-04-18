fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let tmp: Vec<&str> = buf.trim().split("VK").collect(); 

    for s in tmp.iter() {
        if s.len() >= 3 || (s.len() == 2 && s != &"KV") {
            println!("{}", tmp.len());
            return;
        }
    }

    println!("{}", tmp.len()-1);
}
