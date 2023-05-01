use std::fmt::format;

// 太乱啦

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let nums: Vec<i32> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if n == 0 {
        println!("{}", nums[0]);
        return;
    }

    let mut flag = nums[0] == 0;

    print!("{}x{}",
           if_one(nums[0]),
           if n == 1 { "".to_string() } else { format!("^{}", n) }
    );

    for i in 1..(nums.len() - 1) {
        if nums[i] != 0 {
            flag = false;
        } else {
            continue
        }
        if i == nums.len() - 2 {
            print!("{}{}x", if nums[i] > 0 { "+" } else { "" }, if_one(nums[i]));
            continue;
        }

        match nums[i].cmp(&0) {
            std::cmp::Ordering::Less => {
                print!("{}x^{}", if_one(nums[i]), n - i)
            }
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => print!("+{}x^{}", if_one(nums[i]), n - i),
        }
    }

    let tmp = nums[nums.len() - 1];
    match tmp.cmp(&0) {
        std::cmp::Ordering::Less => print!("{}", tmp),
        std::cmp::Ordering::Equal => if flag { print!("0") }
        std::cmp::Ordering::Greater => if flag { print!("{}", tmp) } else { print!("+{}", tmp) }
    }
}

fn if_one(num: i32) -> String {
    if num == 1 { "".to_string() } else if num == -1 { "-".to_string() } else { format!("{}", num) }
}
