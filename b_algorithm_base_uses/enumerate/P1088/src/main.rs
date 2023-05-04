fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let number: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let start_state: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // println!("{}", start_state.len());
    let mut dp_state = Dp_state {
        n,
        add_num: number,
        start_state,
        current_state: vec![0; n],
        is_recording: false,
        cnt: 0,
        is_read: vec![false; n],
        if_continue: true,
    };

    dp(0, &mut dp_state);

    let n = dp_state.current_state.len();
    for (i, num) in dp_state.current_state.iter().enumerate() {
        if i == n - 1 {
            print!("{}", num);
        } else {
            print!("{} ", num);
        }
    }
}

fn dp(i: usize, d: &mut Dp_state) {
    if !d.if_continue {
        return;
    }

    if i == d.n {
        if d.is_recording {
            d.cnt += 1;
        }
        if d.current_state == d.start_state {
            d.is_recording = true;
        }

        if d.cnt == d.add_num {
            d.if_continue = false;
        }
        return;
    }

    for j in d.start_state[i]..d.n + 1 {
        if d.is_read[j - 1] {
            continue;
        }
        d.current_state[i] = j;
        d.is_read[j - 1] = true;
        dp(i + 1, d);
        d.is_read[j - 1] = false;

        if !d.if_continue {
            return;
        }
    }

    d.start_state[i] = 1;
}

struct Dp_state {
    n: usize,
    add_num: usize,
    start_state: Vec<usize>,
    current_state: Vec<usize>,
    is_recording: bool,
    cnt: usize,
    is_read: Vec<bool>,
    if_continue: bool,
}
