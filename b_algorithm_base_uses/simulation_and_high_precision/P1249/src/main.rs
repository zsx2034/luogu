use std::cmp::max as other_max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut n: i32 = buf.trim().parse().unwrap();

    match n {
        3 => {
            println!("1 2");
            println!("2");
            return;
        }
        4 => {
            println!("1 3");
            println!("3");
            return;
        }
        _ => {}
    }

    let mut p = Vec::new();
    let mut tmp = 2;
    while n > 0 {
        n -= tmp;
        p.push(tmp);
        tmp += 1;
    }

    if n < 0 {
        let tmp = -n;
        p.retain(|x| *x != tmp);
    }
    p.iter().for_each(|x| print!("{} ", x));
    println!();


    let mut res = vec![1];
    for i in 0..p.len() {
        mul(&mut res, p[i] as u32);
    }

    let res = res.iter().rev().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    println!("{}", res);
}

fn divide(num: &usize) -> usize {
    if *num % 2 == 0 {
        *num / 2
    } else {
        *num / 2 + 1
    }
}

fn transfer(n: u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut n = n;
    while n > 0 {
        res.push(n % 10);
        n /= 10;
    }
    res
}

fn mul(a: &mut Vec<u32>, b: u32) {
    for i in 0..a.len() {
        a[i] *= b;
    }

    for i in 0..a.len() - 1 {
        if a[i] >= 10 {
            a[i + 1] += a[i] / 10;
            a[i] %= 10;
        }
    }

    while a[a.len() - 1] >= 10 {
        let lp = a.len() - 1;
        let tmp = a[lp] / 10;
        a[lp] %= 10;
        a.push(tmp);
    }
}


#[test]
fn test() {
    let mut a = vec![1];
    mul(&mut a, 2);
    assert_eq!(a, vec![2]);

    let mut a = vec![9];
    mul(&mut a, 2);
    assert_eq!(a, vec![8, 1]);

    let mut a = vec![9, 9];
    mul(&mut a, 2);
    assert_eq!(a, vec![8, 9, 1]);

    let mut a = vec![9, 9, 9];
    mul(&mut a, 2);
    assert_eq!(a, vec![8, 9, 9, 1]);
}