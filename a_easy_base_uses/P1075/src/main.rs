fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let num = buf.trim().parse::<i128>().unwrap();

    let mut b = 0;
    for a in 1..(num as f64).powf(0.5) as i128 + 1 {
        if num % a == 0 {
            b = num / a;
            // if is_prime_number(a) && is_prime_number(b) {
            //     break;
            // }
            // if a <= b && is_prime_number(b) {
            //     break;
            // } else if a > b && is_prime_number(a) {
            //     b = a;
            //     break;
            // }
        }
    }

    println!("{}", b);
}

fn is_prime_number(num: i128) -> bool {
    let upbound = (num as f64).powf(0.5) as i128 + 1;
    for i in 2..upbound {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn calc_all_prime() {
    let mut cnt = 0;
    for i in 2..2e9_f64.powf(0.5) as i128 + 1{
        if is_prime_number(i) {
            println!("{}", i);
            cnt += 1;
        }
    }
    println!();
    println!("{}", cnt);
}