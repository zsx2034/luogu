fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v = buf
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut tmp = 0;

    qsort(&mut v, &mut tmp, 0, n - 1, k);

    println!("{}", tmp);
}

fn qsort(v: &mut Vec<i64>, tmp: &mut i64, l: usize, r: usize, k: usize) {
    if l > r || l > k || r < k {
        return;
    }
    let pivot = v[(l + r) / 2];
    v.swap(l, (l + r) / 2);

    let mut i = l + 1;
    let mut j = r;
    while i < j {
        while i < j && v[i] <= pivot {
            i += 1;
        }
        while i < j && v[j] > pivot {
            j -= 1;
        }

        v.swap(i, j);
    }

    if v[i] > pivot {
        i -= 1;
    }
    v.swap(l, i);
    if i == k {
        *tmp = pivot;
        return ;
    }

    // println!("{:?}, from {l} to {r}", v);
    qsort(v, tmp, l, i.saturating_sub(1), k);
    qsort(v, tmp, i + 1, r, k);
}

