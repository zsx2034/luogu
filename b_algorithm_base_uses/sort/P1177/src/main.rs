use std::ops::Sub;
use std::ptr;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut a: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut tmp = vec![0; n];
    merge_sort(&mut a, &mut tmp, 0, n - 1);

    for num in a {
        print!("{} ", num);
    }
}

fn merge_sort(v: &mut Vec<i32>, tmp: &mut Vec<i32>, l: usize, r: usize) {
    if l >= r {
        return;
    }

    let mid = (l + r) >> 1;
    merge_sort(v, tmp, l, mid);
    merge_sort(v, tmp, mid + 1, r);

    let mut i: usize = l;
    let mut j: usize = mid + 1;
    let mut k: usize = l;

    while i <= mid && j <= r {
        if v[i] <= v[j] {
            tmp[k] = v[i];
            i += 1;
        } else {
            tmp[k] = v[j];
            j += 1;
        }
        k += 1;
    }

    if i <= mid {
        tmp[k..=r].copy_from_slice(&v[i..=mid]);
    }


    unsafe {
        if i <= mid {
            ptr::copy_nonoverlapping(v.as_ptr().add(i), tmp.as_mut_ptr().add(k), mid - i + 1);
        }
        if j <= r {
            ptr::copy_nonoverlapping(v.as_ptr().add(j), tmp.as_mut_ptr().add(k), r - j + 1);
        }
        ptr::copy_nonoverlapping(tmp.as_ptr().add(l), v.as_mut_ptr().add(l), r - l + 1);
    }
}

fn quick_sort(a: &mut Vec<i32>, l: usize, r: usize) {
    if l >= r {
        return;
    }

    let tmp = a[l];
    let mut l_ = l + 1;
    let mut r_ = r;
    let mut tmp_ = 0;
    while l_ < r_ {
        while l_ < r_ && a[l_] <= tmp {
            l_ += 1;
        }
        while l_ < r_ && a[r_] > tmp {
            r_ -= 1;
        }

        tmp_ = a[l_];
        a[l_] = a[r_];
        a[r_] = tmp_;
    }

    if a[l_] <= tmp {
        tmp_ = a[l_];
        a[l_] = a[l];
        a[l] = tmp_;
    } else {
        l_ = l_.saturating_sub(1);
        tmp_ = a[l_];
        a[l_] = a[l];
        a[l] = tmp_;
    }

    quick_sort(a, l, l_.saturating_sub(1));
    quick_sort(a, l_ + 1, r);
}

#[test]
fn test_quick_sort() {
    use std::fs::File;
    use std::io::Read;

    let mut buf = String::new();
    File::open("t.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let s1 = std::time::Instant::now();
    let mut a: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let s2 = std::time::Instant::now();
    let n = a.len() - 1;
    let mut tmp = vec![0; n];
    // quick_sort(&mut a, 0, n);
    // a.sort();
    merge_sort(&mut a, &mut tmp, 0, n);
    let s3 = std::time::Instant::now();

    println!("read time: {:?}", s2.duration_since(s1));
    println!("sort time: {:?}", s3.duration_since(s2));
}
