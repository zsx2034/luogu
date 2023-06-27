use std::borrow::{Borrow, BorrowMut};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize =  iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut m: Vec<Option<Vec<i32>>> = vec![None; n];
    for _ in 0..q {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        match iter.next().unwrap() {
            "1" => {
                let i: usize = iter.next().unwrap().parse().unwrap();
                let j: usize = iter.next().unwrap().parse().unwrap();
                let k: i32 = iter.next().unwrap().parse().unwrap();

                if let Some(v) = m[i].borrow_mut() {
                    if v.len() < j {
                        v.resize(j, 0);
                    } 
                    v[j - 1] = k;
                } else {
                    let mut v = vec![0; j];
                    v[j - 1] = k;
                    m[i] = Some(v);
                }
            },
            "2" => {
                let i: usize = iter.next().unwrap().parse().unwrap();
                let j: usize = iter.next().unwrap().parse().unwrap();
                println!("{}", m[i].as_ref().unwrap()[j - 1]);
            },
            _ => {

            }
        }
    }
}
