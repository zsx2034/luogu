fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    if a <= 0 || b <= 0 || c <= 0 {
        println!("No!!!");
        return;
    }

    solve(a, b, c);
}

fn solve(a: i32, b: i32, c: i32) {
    let mut cnt = 0;
    
    'outer:
    for i in 1..10 {
        for j in 1..10 {
            for k in 1..10 {
                if i != j && j != k && k != i {
                    let tmpa = i * 100 + j * 10 + k;
                    if (tmpa % a != 0) {
                        continue;
                    }
                    let tmpb = tmpa / a * b;
                    let tmpc = tmpa / a * c;

                    if tmpb > 999 || tmpc > 999 {
                        break 'outer;
                    }
                    if (!contains(tmpa, tmpb, tmpc)) {
                        cnt += 1;
                        println!("{} {} {}", tmpa, tmpb, tmpc);
                    }
                }
            }
        }
    }

    if cnt == 0 {
        println!("No!!!");
    }
}

fn contains(a: i32, b: i32, c: i32) -> bool {
    let mut v = vec![false; 11];

    for ch in a.to_string().chars().map(|ch| ch.to_digit(10).unwrap()) {
        if v[ch as usize] {
            return true;
        }
        v[ch as usize] = true;
    }

    for ch in b.to_string().chars().map(|ch| ch.to_digit(10).unwrap()) {
        if v[ch as usize] {
            return true;
        }
        v[ch as usize] = true;
    }

    for ch in c.to_string().chars().map(|ch| ch.to_digit(10).unwrap()) {
        if v[ch as usize] {
            return true;
        }
        v[ch as usize] = true;
    }

    if v[0] {
        return true;
    }
    return false;
}
