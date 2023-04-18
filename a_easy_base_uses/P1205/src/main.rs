fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let mut ori: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let tmp: Vec<char> = buf.trim().chars().into_iter().collect();
        ori.push(tmp);
    }

    let mut latest: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let tmp: Vec<char> = buf.trim().chars().into_iter().collect();
        latest.push(tmp);
    }

    if check_same(&method_1(&ori, n), &latest, n) {
        print!("{}", 1);
        return;
    }
    if check_same(&method_2(&ori, n), &latest, n) {
        print!("{}", 2);
        return;
    }
    if check_same(&method_3(&ori, n), &latest, n) {
        print!("{}", 3);
        return;
    }
    if check_same(&method_4(&ori, n), &latest, n) {
        print!("{}", 4);
        return;
    }
    if method_5(&ori, &latest, n) {
        print!("{}", 5);
        return;
    }

    if check_same(&method_6(&ori, n), &latest, n) {
        print!("{}", 6);
        return;
    }

    print!("{}", 7);
}

/// rotatation of an array of chars 90 degrees counterclockwise
fn method_1(ori: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut latest = ori.clone();
    let mut ori_line = n;
    let mut ori_row = 0;
    let mut latest_line = 0;
    let mut latest_row = 0;

    for _ in 0..n {
        ori_line = n;
        latest_row = 0;
        for _ in 0..n {
            ori_line -= 1;
            latest[latest_line][latest_row] = ori[ori_line][ori_row];
            latest_row += 1;
        }

        ori_row += 1;
        latest_line += 1;
    }
    return latest;
}

/// rotatation of an array of chars 180 degrees counterclockwise
fn method_2(ori: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut latest = ori.clone();
    let mut ori_line = n;
    let mut ori_row = n;
    let mut latest_line = 0;
    let mut latest_row = 0;

    for _ in 0..n {
        ori_row = n;
        latest_row = 0;
        ori_line -= 1;
        for _ in 0..n {
            ori_row -= 1;
            latest[latest_line][latest_row] = ori[ori_line][ori_row];
            latest_row += 1;
        }

        latest_line += 1;
    }
    return latest;
}

/// rotatation of an array of chars 270 degrees counterclockwise
fn method_3(ori: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut latest = ori.clone();
    let mut ori_line = 0;
    let mut ori_row = n;
    let mut latest_line = 0;
    let mut latest_row = 0;

    for _ in 0..n {
        ori_line = 0;
        latest_row = 0;
        ori_row -= 1;
        for _ in 0..n {
            latest[latest_line][latest_row] = ori[ori_line][ori_row];
            ori_line += 1;
            latest_row += 1;
        }
        latest_line += 1;
    }
    return latest;
}

// horizantal flip
fn method_4(ori: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut latest = ori.clone();
    let mut ori_line = 0;
    let mut ori_row = n;
    let mut latest_line = 0;
    let mut latest_row = 0;

    for _ in 0..n {
        ori_row = n;
        latest_row = 0;
        for _ in 0..n {
            ori_row -= 1;
            latest[latest_line][latest_row] = ori[ori_line][ori_row];
            latest_row += 1;
        }
        ori_line += 1;
        latest_line += 1;
    }
    return latest;
}

// combinations of previous methods
fn method_5(ori: &Vec<Vec<char>>, latest: &Vec<Vec<char>>, n: usize) -> bool {
    let mut tmp_flip = method_4(ori, n);

    let tmp = method_1(&tmp_flip, n);
    if check_same(&tmp, latest, n) {
        return true;
    }

    let tmp = method_2(&tmp_flip, n);
    if check_same(&tmp, latest, n) {
        return true;
    }

    let tmp = method_3(&tmp_flip, n);
    if check_same(&tmp, latest, n) {
        return true;
    }

    return false;
}

// no changes
fn method_6(ori: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    ori.clone()
}

// check if ori and latest are equal or not
fn check_same(ori: &Vec<Vec<char>>, latest: &Vec<Vec<char>>, n: usize) -> bool {
    for i in 0..n {
        for j in 0..n {
            if ori[i][j] != latest[i][j] {
                return false;
            }
        }
    }

    return true;
}