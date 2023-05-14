fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut i: usize = 0;
    let mut p: Vec<u64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut ans = 0;

    let mut intervals: Vec<Interval> = vec![Interval::new(0, p.len() - 1)];
    while let Some(interval) = intervals.pop() {
        let min = min(&p, interval.start, interval.end);
        ans += min;
        let mut i = interval.start;
        while i <= interval.end {
            p[i] -= min;
            if p[i] == 0 {
                i += 1;
            } else {
                let mut j = i + 1;
                while j <= interval.end {
                    p[j] -= min;
                    if p[j] == 0 {
                        break;
                    }
                    j += 1;
                }
                intervals.push(Interval::new(i, j - 1));
                i = j + 1;
            }
        }
    }

    println!("{}", ans);
}

fn min(v: &Vec<u64>, start: usize, end: usize) -> u64 {
    let mut min = v[start];
    for i in start..=end {
        if v[i] < min {
            min = v[i];
        }
    }
    min
}

struct Interval {
    // equal
    start: usize,
    // equal
    end: usize,
}

impl Interval {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
