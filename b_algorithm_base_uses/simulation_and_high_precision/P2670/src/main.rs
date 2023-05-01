fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut res = vec![vec![0; m]; n];
    let mut graph = Vec::new();
    for i in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        graph.push(buf.trim().chars().collect::<Vec<char>>());
    }

    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == '*' {
                count(&mut res, i, j);
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if graph[i][j] == '*' {
                print!("*");
            } else {
                print!("{}", res[i][j]);
            }
        }
        println!();
    }
}

// add one to the postion around the (i, j) exclude the (i, j)
fn count(res: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    let n = res.len();
    let m = res[0].len();
    for x in i.saturating_sub(1)..=i + 1 {
        for y in j.saturating_sub(1)..=j + 1 {
            if x < n && y < m && (x != i || y != j) {
                res[x][y] += 1;
            }
        }
    }
}
