/// 6tian
/// 1 2 2 3 3 3

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let days: i64 = buf.trim().parse().unwrap();
    let mut golds = 0;
    let mut standard = 1;
    let mut end = 0;
    let mut today = 1;

    while  today <= days{
        end = standard + today;
        while today < end && today < days + 1 {
            golds += standard;  
            today += 1;
        }
        standard += 1;
    }

    println!("{}", golds);
}
