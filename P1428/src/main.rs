use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i32>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let data: Vec<_> = buf.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();

    let mut record: HashMap<i32, i32> = HashMap::new();
    let mut res = 0;
    for num in data {
        for key in record.keys() {
            if key < &num {
                res += record.get(key).unwrap();
            }
        }

        if let Some(cnt) = record.get_mut(&num) {
            *cnt += 1;
        } else {
            record.insert(num, 1);
        }

        print!("{} ", res);
        res = 0;
    }
}

fn solution2() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i32>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let data: Vec<_> = buf.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
    let mut res: Vec<i32> = vec![0;data.len()];

    for i in 0..data.len() {
        for j in 0..i {
            if data[j] < data[i] {
                res[i] += 1;
            }
        }
    }

    for num in res {
        print!("{} ", num);
    }
}

fn solution1() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i32>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let data: Vec<_> = buf.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
    
    let mut res = 0;
    for i in 0..data.len() {
        for j in 0..i {
            if data[j] < data[i] {
                res += 1;
            }
        }
        print!("{} ", res);
        res = 0;
    }
}
