use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut viariables: HashMap<String, Option<i32>> = HashMap::new();
    let buf = buf.trim().chars().collect::<Vec<char>>();
    // 0 - start, 1 - left, 2 - right_val, 3 - equal, 4 - right_viariable, 5 - end
    let mut state = 0;

    let mut start_left = 0;
    let mut end_left = 0;
    let mut start_right = 0;
    let mut end_right = 0;
    let mut assign = false;
    for (i, ch) in buf.iter().enumerate() {
        match state {
            0 => {
                if ch.is_alphabetic() {
                    state = 1;
                    start_left = i;
                } else if ch == &' ' || ch == &';' {
                    // keep state
                } else {
                    state = 5;
                }
            }
            // 1 - left
            1 => {
                if ch.is_alphabetic() {
                    // keep state
                } else if ch == &':' || ch == &' ' {
                    end_left = i;
                    state = 3;
                } else if ch == &';' {
                    end_left = i;
                    viariables
                        .entry(buf[start_left..end_left].iter().collect())
                        .and_modify(|val| *val = None)
                        .or_insert(None);
                    state = 0;
                } else {
                    state = 5;
                }
            }
            // 2 - right_val
            2 => {
                if ch.is_ascii_digit() {
                    // keep state
                } else if ch == &' ' || ch == &';' {
                    end_right = i;
                    viariables
                        .entry(buf[start_left..end_left].iter().collect())
                        .and_modify(|val| {
                            *val = Some(
                                buf[start_right..end_right]
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i32>()
                                    .unwrap(),
                            )
                        })
                        .or_insert(Some(
                            buf[start_right..end_right]
                                .iter()
                                .collect::<String>()
                                .parse::<i32>()
                                .unwrap(),
                        ));
                    state = 0;
                }
            }
            // 3 - equal
            3 => {
                if ch.is_alphabetic() {
                    state = 4;
                    start_right = i;
                } else if ch.is_ascii_digit() {
                    state = 2;
                    start_right = i;
                } else if ch == &'=' || ch == &' ' || ch == &':' {
                    // keep state
                } else {
                    state = 5;
                }
            }
            // 4 - right_viariable
            4 => {
                if ch.is_alphabetic() {
                    // keep state
                } else if ch == &' ' || ch == &';' {
                    end_right = i;
                    let tmp = *viariables
                        .get(&(buf[start_right..end_right].iter().collect::<String>()))
                        .unwrap();
                    viariables
                        .entry(buf[start_left..end_left].iter().collect())
                        .and_modify(|val| {
                            *val = tmp;
                        })
                        .or_insert(tmp);
                    state = 0;
                } else {
                    state = 5;
                }
            }
            // 5 - end
            5 => {
                break;
            }
            _ => {}
        }
    }

    print!("{}", viariables["a"].unwrap_or(0));
    print!(" {}", viariables["b"].unwrap_or(0));
    print!(" {}", viariables["c"].unwrap_or(0));
}
