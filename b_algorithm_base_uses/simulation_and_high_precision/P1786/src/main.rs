use std::collections::HashMap;
use std::ops::Sub;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    // 一位BangZhu，两位FuBangZhu，两位HuFa，四位ZhangLao，七位TangZhu，二十五名JingYing，BangZhong若干
    // 等级依次降序，帮主最高，帮众最低
    let mut PROF: HashMap<String, usize> = HashMap::from([
        ("BangZhu".to_string(), 1),
        ("FuBangZhu".to_string(), 2),
        ("HuFa".to_string(), 2),
        ("ZhangLao".to_string(), 4),
        ("TangZhu".to_string(), 7),
        ("JingYing".to_string(), 25),
        ("BangZhong".to_string(), 2 * n),
    ]);

    let PROF_LEVEL: Vec<&str> = Vec::from([
        "BangZhu", "FuBangZhu", "HuFa", "ZhangLao", "TangZhu", "JingYing", "BangZhong"
    ]);


    let mut players = Vec::with_capacity(n);
    for i in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let name = iter.next().unwrap().to_string();
        let tmp_prof_level = iter.next().unwrap();
        let prof_level = PROF_LEVEL.iter().position(|&x| x == tmp_prof_level).unwrap();
        let bang_gong = iter.next().unwrap().parse().unwrap();
        let level = iter.next().unwrap().parse().unwrap();

        if prof_level <= 1 {
            println!("{} {} {}", name, tmp_prof_level, level);
        } else {
            players.push(Player::new(i, name, prof_level, bang_gong, level));
        }
    }

    players.sort_by(|a, b| {
        if a.bang_gong == b.bang_gong {
            a.id.cmp(&(b.id))
        } else {
            b.bang_gong.cmp(&(a.bang_gong))
        }
    });

    for i in 0..players.len() {
        match i {
            0..=1 => {
                players[i].prof_level = 2;
            }
            2..=5 => {
                players[i].prof_level = 3;
            }
            6..=12 => {
                players[i].prof_level = 4;
            }
            13..=37 => {
                players[i].prof_level = 5;
            }
            _ => {
                players[i].prof_level = 6;
            }
        }
    }

    players.sort_by(|a, b| {
        if a.prof_level == b.prof_level {
            if b.level == a.level {
                a.id.cmp(&(b.id))
            } else {
                b.level.cmp(&(a.level))
            }
        } else {
            a.prof_level.cmp(&(b.prof_level))
        }
    });

    for player in players.iter() {
        println!("{} {} {}", player.name, PROF_LEVEL[player.prof_level], player.level);
    }
}

#[derive(Debug)]
struct Player {
    id: usize,
    name: String,
    prof_level: usize,
    bang_gong: i128,
    level: usize,
}

impl Player {
    pub fn new(id: usize, name: String, prof_level: usize, bang_gong: i128, level: usize) -> Self {
        Player {
            id,
            name,
            prof_level,
            bang_gong,
            level,
        }
    }
}