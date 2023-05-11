use std::vec;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().unwrap();
    let b: i64 = iter.next().unwrap().parse().unwrap();
    let c: i64 = iter.next().unwrap().parse().unwrap();
    let d: i64 = iter.next().unwrap().parse().unwrap();

    let mut map = vec![vec![0; b as usize + 1]; a as usize + 1];
    map[0][0] = 0;
    map[1][0] = 1;
    map[0][1] = 1;
    dp(&mut map, a, b, c, d);

    println!("{}", map[a as usize][b as usize]);

}

fn dp(map: &mut Vec<Vec<u64>>, cur_x: i64, cur_y: i64, horse_x: i64, horse_y: i64) -> u64 {
    if cur_x < 0 || cur_y < 0 {
        return 0;
    }

    if cur_x == horse_x && cur_y == horse_y {
        return 0;
    }

    let tmpx = (cur_x - horse_x).abs();
    let tmpy = (cur_y - horse_y).abs();
    if (tmpx == 1 && tmpy == 2) || (tmpx == 2 && tmpy == 1) {
        return 0;
    }



    if map[cur_x as usize][cur_y as usize] != 0 {
        return map[cur_x as usize][cur_y as usize];
    }

    map[cur_x as usize][cur_y as usize] =
        dp(map, cur_x - 1, cur_y, horse_x, horse_y) + dp(map, cur_x, cur_y - 1, horse_x, horse_y);

    return map[cur_x as usize][cur_y as usize];
}

struct DpState {
    cnt: u64,
    target_x: i64,
    target_y: i64,
    // v: Vec<Vec<i64>>,
    // tmp: Vec<Vec<i64>>
    horse_x: i64,
    horse_y: i64,
}
