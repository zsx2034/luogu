use std::{ops::IndexMut, vec};

static TEMPLATE: [[[char; 3]; 5]; 10] = [
    [
        //0
        ['X', 'X', 'X'],
        ['X', '.', 'X'],
        ['X', '.', 'X'],
        ['X', '.', 'X'],
        ['X', 'X', 'X'],
    ],
    [
        //1
        ['.', '.', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
    ],
    [
        //2
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['X', 'X', 'X'],
        ['X', '.', '.'],
        ['X', 'X', 'X'],
    ],
    [
        //3
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['X', 'X', 'X'],
    ],
    [
        //4
        ['X', '.', 'X'],
        ['X', '.', 'X'],
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
    ],
    [
        //5
        ['X', 'X', 'X'],
        ['X', '.', '.'],
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['X', 'X', 'X'],
    ],
    [
        //6
        ['X', 'X', 'X'],
        ['X', '.', '.'],
        ['X', 'X', 'X'],
        ['X', '.', 'X'],
        ['X', 'X', 'X'],
    ],
    [
        //7
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
        ['.', '.', 'X'],
    ],
    [
        //8
        ['X', 'X', 'X'],
        ['X', '.', 'X'],
        ['X', 'X', 'X'],
        ['X', '.', 'X'],
        ['X', 'X', 'X'],
    ],
    [
        //9
        ['X', 'X', 'X'],
        ['X', '.', 'X'],
        ['X', 'X', 'X'],
        ['.', '.', 'X'],
        ['X', 'X', 'X'],
    ],
];

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<usize>().unwrap();

    let width = n * 3 + n - 1;
    let mut res = vec![vec!['X'; width]; 5];

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");

    let mut start = 0;
    let buf = buf.trim();
    let temp: Vec<usize> = buf
        .trim()
        .split("")
        .filter(|x| !(*x).is_empty())
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    for (cnt, index) in temp.iter().enumerate() {
        for i in 0..5 {
            for j in 0..3 {
                res[i][start + j] = TEMPLATE[*index][i][j];
            }
        }
        if cnt + 1 != temp.len() {
            for j in 0..5 {
                res[j][start + 3] = '.';
            }
        }

        start += 4;
    }
    for line in res {
        for item in line {
            print!("{}", item);
        }
        println!();
    }
}
