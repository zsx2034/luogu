fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let t: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let sx: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let sy: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let gx: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let gy: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;


    let mut map = vec![vec![true; m]; n];

    for _ in 0..t {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        map[x - 1][y - 1] = false;
    }

    let mut cnt = 0;
    let mut walked = vec![vec![false; m]; n];
    dp(&map, &mut walked, gx, gy, sx, sy, &mut cnt);
    println!("{}", cnt);
}

fn dp(map: &Vec<Vec<bool>>, walked: &mut Vec<Vec<bool>>, gx: usize, gy: usize, x: usize, y: usize, cnt: &mut usize) {
    if !map[x][y] || walked[x][y] {
        return;
    }

    if x == gx && y == gy {
        *cnt += 1;
        return;
    }

    walked[x][y] = true;
    if x < map.len() - 1 {
        dp(map, walked, gx, gy, x + 1, y, cnt);
    }
    if x > 0 {
        dp(map, walked, gx, gy, x - 1, y, cnt);
    }
    if y < map[0].len() - 1 {
        dp(map, walked, gx, gy, x, y + 1, cnt);
    }
    if y > 0 {
        dp(map, walked, gx, gy, x, y - 1, cnt);
    }
    walked[x][y] = false;
}
