fn main() {
    let v = vec![6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut cnt = 0;
    // // a+a=b
    // for i in 0..100 {
    //     let mut tmp = 0;
    //     for num in i.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
    //         tmp += v[num as usize];
    //     }

    //     tmp *= 2;
    //     for num in (2 * i).to_string().chars().map(|c| c.to_digit(10).unwrap()) {
    //         tmp += v[num as usize];
    //     }

    //     tmp += 4;
    //     if tmp == n {
    //         cnt += 1;
    //     }
    // }

    for i in 0..100 {
        for j in 0..100 {
            // if i == j {
            //     continue;
            // }

            let mut tmp = 0;
            for num in i.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
                tmp += v[num as usize];
            }

            for num in j.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
                tmp += v[num as usize];
            }

            for num in (i + j).to_string().chars().map(|c| c.to_digit(10).unwrap()) {
                tmp += v[num as usize];
            }

            tmp += 4;
            if tmp == n {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}

fn solve(n: usize) {
    let v = vec![6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).unwrap();
    // let n: usize = buf.trim().parse().unwrap();

    let mut cnt = 0;
    // // a+a=b
    // for i in 0..100 {
    //     let mut tmp = 0;
    //     for num in i.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
    //         tmp += v[num as usize];
    //     }

    //     tmp *= 2;
    //     for num in (2 * i).to_string().chars().map(|c| c.to_digit(10).unwrap()) {
    //         tmp += v[num as usize];
    //     }

    //     tmp += 4;
    //     if tmp == n {
    //         cnt += 1;
    //     }
    // }

    for i in 0..1000 {
        for j in 0..1000 {
            // if i == j {
            //     continue;
            // }

            let mut tmp = 0;
            for num in i.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
                tmp += v[num as usize];
            }

            for num in j.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
                tmp += v[num as usize];
            }

            for num in (i + j).to_string().chars().map(|c| c.to_digit(10).unwrap()) {
                tmp += v[num as usize];
            }

            tmp += 4;
            if tmp == n {
                cnt += 1;
                // println!("{} + {} = {}", i, j, i + j);
            }
        }
    }

    println!("{}", cnt);
}

fn solve2(n: usize) {
    let v = vec![6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut cnt = 0;

    for  i in 0..1000 {
        for  j in 0..1000 {

            let mut tmp = 0;
            let mut tmpi = i;
            let mut tmpj = j;
            let mut sum = i + j;

            if tmpi == 0 {
                tmp += v[0];
            }
            while tmpi > 0 {
                tmp += v[tmpi % 10];
                tmpi /= 10;
            }


            if tmpj == 0 {
                tmp += v[0];
            }
            while tmpj > 0 {
                tmp += v[tmpj % 10];
                tmpj /= 10;
            }

            if sum == 0 {
                tmp += v[0];
            }
            while sum > 0 {
                tmp += v[sum % 10];
                sum /= 10;
            }

            tmp += 4;
            if tmp == n {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
