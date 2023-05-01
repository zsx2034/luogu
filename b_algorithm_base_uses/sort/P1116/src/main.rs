fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v: Vec<i32> = Vec::with_capacity(n);
    let mut len = 0;
    while len < n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let tmp = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        unsafe {
            v.set_len(len + tmp.len());
            std::ptr::copy_nonoverlapping(tmp.as_ptr(), v.as_mut_ptr().add(len), tmp.len());
            len = v.len();
        }
    }

    let mut cnt = 0;
    for _ in 0..n {
        let mut f = true;
        for j in 0..n - 1 {
            if v[j] > v[j + 1] {
                f = false;
                cnt += 1;
                v.swap(j, j + 1);
            }
        }
        if f {
            break;
        }
    }

    println!("{}", cnt);
}