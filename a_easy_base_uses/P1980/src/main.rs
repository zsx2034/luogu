fn main() {
    clac();
}

fn emulate() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("mgs");
    let mut input = buf.split_whitespace();
    let upbound = input.next().unwrap().parse::<i32>().unwrap();
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


/// 计算思想
/// 对于 0 - 100 之间的数, 0 出现的次数可分为
/// 出现在个位：0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100
///     每 10 个数个位会进行一次从 0-9 的循环， 即共有 100/10*1=10 100%10-(0*100/10-1)=1
/// 出现在十位：100
///     每 100 个数会有 01-09 的循环，但是，这并不算入统计
/// 出现在百位：
/// 共11个
fn clac() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("mgs");
    let mut buf = buf.split_whitespace();
    let mut upbound = buf.next().unwrap().parse::<i32>().unwrap();
    let target = buf.next().unwrap().parse::<i32>().unwrap();

    let mut cnt = 0;
    let mut div = 10;
    let flag = if target == 0 { 1 } else { 10 };
    while upbound % (div / flag) != upbound {
        cnt += (upbound / div - if target == 0 { 1 } else { 0 } ) * div / 10;
        let tmp = match div {
            10 => {
                if upbound % div >= target { 1 } else { 0 }
            },
            _ => {
                upbound % div - target * div / 10 + 1
            }
        };
        cnt += if tmp < 0 { 0 } else { tmp };
        div *= 10;
    }

    println!("{}", cnt);
}
