fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let nums: Vec<u8> = buf
        .trim()
        .split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let map_len = nums.len();
    print!("{} ", map_len);
    let mut sign = 0;
    let mut cnt = 0;
    nums.iter().for_each(|x| {
        if x == &sign {
            cnt += 1;
        } else {
            print!("{} ", cnt);
            cnt = 1;
            sign = *x;
        }
    });
    for _ in 0..map_len - 1 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf
            .trim()
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u8>().unwrap())
            .for_each(|x| {
                if x == sign {
                    cnt += 1;
                } else {
                    print!("{} ", cnt);
                    cnt = 1;
                    sign = x;
                }
            });
    }
    print!("{}", cnt);
}
