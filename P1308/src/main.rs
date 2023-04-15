fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf = buf.to_ascii_lowercase();
    let template = buf.trim().chars().collect::<Vec<char>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf = buf.to_ascii_lowercase();
    buf.pop();
    let words: Vec<char> = buf.chars().collect();

    let mut state = false;
    let mut word_start = 0;

    let mut res_pos = -1;
    let mut res_cnt = 0;
    for (i, ch) in words.iter().enumerate() {
        if ch == &' ' {
            //  word end
            if state {
                let word = &words[word_start..i];
                if word == template {
                    res_cnt += 1;
                    if res_pos == -1 {
                        res_pos = word_start as i32;
                    }
                }
            }
            // continuious space
            else {
            }
            state = false;
        } else {
            // word start
            if !state {
                word_start = i;
            }
            // word continue
            else {
            }
            state = true;
        }
    }

    if res_cnt == 0 {
        println!("-1");
    } else {
        println!("{} {}", res_cnt, res_pos);
    }
}
