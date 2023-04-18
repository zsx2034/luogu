use std::collections::HashMap;

fn main() {
    let mut record: HashMap<char, i32> = HashMap::new();

    let mut buf = String::new();

    for _ in 0..4 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        for c in buf.chars() {
            record.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let mut max = 0;
    for (k, v) in record.iter() {
        if v > &max  && *k <= 'Z' && *k >= 'A'{
            max = *v;
        }
    }

    let mut res = vec![vec![' '; 26]; max as usize + 1];

    // fill the first line with alphabet
    for i in 0..26 {
        res[max as usize][i] = (b'A' + i as u8) as char;
    }

    // fill the table based on the record
    for i in 0..26 {
        let c = (b'A' + i as u8) as char;
        if let Some(v) = record.get(&c) {
            for j in 0..*v {
                res[max as usize - j as usize - 1][i] = '*';
            }
        }
    }

    // remove space at the end of each line
    for i in 0..max as usize {
        for j in (0..26).rev() {
            if res[i][j] == ' ' {
                res[i][j] = '-';
            } else {
                break;
            }
        }
    }

    for i in 0..max as usize + 1 {
        for j in 0..26 {
            if res[i][j] != '-' {
                if j != 25 && res[i][j + 1] != '-' {
                    print!("{} ", res[i][j]);
                } else {
                    print!("{}", res[i][j]);
                }
            } else {
                break;
            }
        }
        println!();
    }

}
