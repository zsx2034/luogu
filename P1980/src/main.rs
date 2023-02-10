fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("mgs");
    let mut input = buf.split_whitespace();
    let mut upbound = input.next().unwrap().parse::<i32>().unwrap();
    let target = input.next().unwrap().parse::<i32>().unwrap();

    let mut res = 0;
    for mut num in 1..upbound + 1 {
        while num > 0 {
            if num % 10 == target {
                res += 1;
            }
            num /= 10;
        } 
    }

    println!("{}", res);
}

fn clac() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("mgs");
    let mut input = buf.split_whitespace();
    let mut upbound = input.next().unwrap().parse::<i32>().unwrap();
    let target = input.next().unwrap().parse::<i32>().unwrap();

    let mut cnt = 0;
    let mut div = 10;
    while upbound % (div / 10) != upbound {
        cnt += upbound / div * div / 10;
        let tmp = upbound % div - target * div / 10 + 1;
        cnt += if tmp < 0 { 0 } else { tmp };
        div *= 10;
    }

    if target == 0 {
        cnt -= 1;
    }

    println!("{}", cnt);
}
