fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v: Vec<Option<Time>> = vec![None; 100_0001];
    for i in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let time = buf.split_whitespace().collect::<Time>();
        let i = time.end as usize;
        if let Some(t) = &v[i] {
            if t.start < time.start {
                v[i] = Some(time);
            }
        } else {
            v[i] = Some(time);
        }
    }

    let v: Vec<Time> = v.into_iter().flatten().collect();

    let mut ans = 0;
    let mut end = 0;
    for t in v {
        if end <= t.start {
            ans += 1;
            end = t.end;
        } else if end > t.end {
            end = t.end;
        }
    }

    println!("{}", ans);
}

#[derive(Clone, Debug)]
struct Time {
    start: i32,
    end: i32,
}

impl Time {
    fn new(start: i32, end: i32) -> Time {
        Time { start, end }
    }
}

impl<'a> FromIterator<&'a str> for Time {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let start = iter.next().unwrap().parse().unwrap();
        let end = iter.next().unwrap().parse().unwrap();
        Time { start, end }
    }
}
