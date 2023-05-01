fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut long: Vec<u32> = buf
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut short = buf
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    if long.len() < short.len() {
        std::mem::swap(&mut long, &mut short);
    }

    let mut res = vec![0;long.len() + 1];
    let mut long_iter = long.iter().rev();
    let mut short_iter = short.iter().rev();
    let mut res_iter = res.iter_mut().rev();
    for _ in 0..short.len() {
        let sum: u32 = long_iter.next().unwrap() + short_iter.next().unwrap();
        *res_iter.next().unwrap() += sum;
    }   
    for num in long_iter {
        *res_iter.next().unwrap() += num;
    }

    for i in (0..res.len()).rev() {
        if res[i] >= 10 {
            res[i - 1] += res[i] / 10;
            res[i] %= 10;
        }
    }

    let mut flag = true;
    for num in res.iter() {
        if  flag && num == &0{
            continue;
        } else {
            flag = false;
            print!("{}", num);
        }
    }

    if flag {
        print!("0");
    }
}
