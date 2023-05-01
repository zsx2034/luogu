use std::collections::VecDeque;
use std::vec;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let args: Vec<i32> = buf.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();

    let chars: Vec<char> = buf.trim().chars().collect();

    let mut record = VecDeque::new();
    for (pos, ch) in chars.iter().enumerate() {
        if ch == &'-' {
            record.push_back(pos);
        }
    }

    let mut p = 0;
    while !record.is_empty() {
        let tmp = record.pop_front().unwrap();
        for i in p..tmp {
            print!("{}", chars[i]);
        }
        if tmp != chars.len() - 1 && tmp != 0 {
            expand(chars[tmp - 1], chars[tmp + 1], &args);
        } else {
            print!("-");
        }
        p = tmp + 1;
    }

    for i in p..chars.len() {
        print!("{}", chars[i]);
    }
}

fn expand(l: char, r: char, args: &Vec<i32>) {

    if l as u8 >= r as u8 {
        print!("-");
        return;
    }

    if !(l.is_alphabetic() && r.is_alphabetic()) && !( l.is_numeric() && r.is_numeric()) {
        print!("-");
        return;
    }

    let mut range: Vec<char> = (l..r).skip(1).collect();

    if args[2] == 1 {} else {
        range.reverse();
    }

    for i in range {
        for _ in 0..args[1] {
            match args[0] {
                1 => {
                    print!("{}", i.to_lowercase());
                }
                2 => {
                    print!("{}", i.to_uppercase());
                }
                3 => {
                    print!("*");
                }
                _ => {}
            }
        }
    }
}


#[test]
fn t() {
    assert_eq!(false, '-'.is_alphabetic());
    assert_eq!(true, '1'.is_numeric());
}