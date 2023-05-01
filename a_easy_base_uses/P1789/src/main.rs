static TEMPLATE_FIRE: [[bool; 5]; 5] = [
    [false, false, true, false, false],
    [false, true,  true, true,  false],
    [true,  true,  true, true,  true],
    [false, true,  true, true,  false],
    [false, false, true, false, false],
];

static TEMPLATE_STONE: [[bool; 5]; 5] = [
    [true, true, true, true, true],
    [true, true, true, true, true],
    [true, true, true, true, true],
    [true, true, true, true, true],
    [true, true, true, true, true],
];

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut input = buf
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap());

    let n = input.next().unwrap();
    let m = input.next().unwrap();
    let k = input.next().unwrap();

    let mut map = vec![vec![false; n]; n];

    for _ in 0..m {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let pos = buf
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap() - 3)
            .collect::<Vec<i32>>();

        fire(&pos, n as i32, &mut map);
    }

    for _ in 0..k {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let pos = buf
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap() - 3)
            .collect::<Vec<i32>>();

        stone(&pos, n as i32, &mut map);
    }

    let mut cnt = 0;
    for line in map {
        for item in line {
            if !item {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}

fn fire(pos: &[i32], n: i32, map: &mut Vec<Vec<bool>>) {
    for i in 0..5 { // y
        if pos[1] + i < 0 || pos[1] + i >= n {
            continue;
        }
        for j in 0..5 { // x
            if pos[0] + j < 0 || pos[0] + j >= n {
                continue;
            }
            map[(pos[1] + i) as usize][(pos[0] + j) as usize] |= TEMPLATE_FIRE[i as usize][j as usize];
        }
    }
}

fn stone(pos: &[i32], n: i32, map: &mut Vec<Vec<bool>>) {
    for i in 0..5 { // y
        if pos[1] + i < 0 || pos[1] + i >= n {
            continue;
        }
        for j in 0..5 { // x
            if pos[0] + j < 0 || pos[0] + j >= n {
                continue;
            }
            map[(pos[1] + i) as usize][(pos[0] + j) as usize] |= TEMPLATE_STONE[i as usize][j as usize];
        }
    }
}