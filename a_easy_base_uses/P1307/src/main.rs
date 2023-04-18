fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut nums = buf.trim().parse::<i64>().unwrap();
    let mut res = 0;
    if nums < 0 {
        nums = -nums;
        while nums > 0 {
            res = res * 10 + nums % 10;
            nums /= 10;
        }
        println!("{}", -res);
    } else {
        while nums > 0 {
            res = res * 10 + nums % 10;
            nums /= 10;
        }
        println!("{}", res);
    }
}
