fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    // 所有物品的加工安排顺序
    let mut sequence: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    sequence.iter_mut().for_each(|x| { *x -= 1; });

    // 每台机器的可以使用的最后时间
    let mut ready_time: Vec<usize> = vec![0; m];

    let mut tmp: Vec<usize> = Vec::new();
    // 每件物品的加工顺序
    let mut sequence_per_item: Vec<Vec<usize>> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace().map(|x| x.parse().unwrap()).for_each(|x| tmp.push(x));
    }

    let mut iter = tmp.iter();
    for i in 0..n {
        let mut tmp2 = Vec::new();
        for _ in 0..m {
            tmp2.push(*iter.next().unwrap());
        }
        sequence_per_item.push(tmp2);
    }

    tmp.clear();
    let mut times: Vec<Vec<usize>> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace().map(|x| x.parse().unwrap()).for_each(|x| tmp.push(x));
    }

    let mut iter = tmp.iter();
    for i in 0..n {
        let mut tmp2 = Vec::new();
        for _ in 0..m {
            tmp2.push(*iter.next().unwrap());
        }
        times.push(tmp2);
    }

    // 每件物品的当前需要处理的工序
    let mut p: Vec<usize> = vec![0; n];

    // 每台机器每段空闲时间
    let mut square_time: Vec<Vec<SquareTime>> = vec![Vec::new(); m];
    let mut end_time = vec![0; n];
    for (when, i) in sequence.iter().enumerate() {
        let i = *i;
        // 当前物品的当前工序对应的机器
        let machine = sequence_per_item[i][p[i]] - 1;
        // 当前物品的当前工序对应的时间
        let time = times[i][p[i]];

        // 当前物品的上一工序对应的完成
        let last_end_time = end_time[i];
        // 检查是否有空闲时间段可以安排
        let mut need_alloc = true;
        let mut tmp = None;
        for st in square_time[machine].iter_mut() {
            // 发现可以安排的空闲时间
            if st.start_time >= last_end_time && st.time_len >= time {
                *st = SquareTime {
                    start_time: st.start_time + time,
                    time_len: st.time_len - time,
                };
                end_time[i] = st.start_time;
                // println!("item: {}, step: {}, machine: {}, start time: {}, end time: {}, time len: {}, square time: 1", i + 1, p[i], machine + 1, end_time[i] - time, end_time[i], time);
                need_alloc = false;
                break;
            }

            // TODO: Attention: 遗忘该种情况
            if st.start_time < last_end_time && st.time_len + st.start_time >= last_end_time + time {
                tmp = Some(SquareTime {
                    start_time: st.start_time,
                    time_len: last_end_time - st.start_time,
                });
                *st = SquareTime {
                    start_time: last_end_time + time,
                    time_len: st.time_len + st.start_time - last_end_time - time,
                };

                end_time[i] = st.start_time;
                // println!("item: {}, step: {}, machine: {}, start time: {}, end time: {}, time len: {}, square time: 1", i + 1, p[i], machine + 1, end_time[i] - time, end_time[i], time);
                need_alloc = false;
                break;
            }
        }

        if let Some(inner) = tmp {
            let mut pos = 0;
            for (tp, st) in square_time[machine].iter().enumerate() {
                if st.start_time >= inner.start_time {
                    pos = tp;
                    break;
                }
            }
            square_time[machine].insert(pos, inner);
        }

        // 没有空闲时间段可以安排，则需要直接在后面分配时间
        if need_alloc {
            // 如果当前机器的就绪时间短于上一工序的完成时间，则会产生空闲时间
            if ready_time[machine] < last_end_time {
                square_time[machine].push(SquareTime {
                    start_time: ready_time[machine],
                    time_len: last_end_time - ready_time[machine],
                });
                ready_time[machine] = last_end_time;
            }

            ready_time[machine] += time;
            end_time[i] = ready_time[machine];
            // println!("item: {}, step: {}, machine: {}, start time: {}, end time: {}, time len: {}, square time: 0", i + 1, p[i], machine + 1, ready_time[machine] - time, end_time[i], time);
        }

        p[i] += 1;
    }

    print!("{}", end_time.iter().max().unwrap());
}

#[derive(Copy, Clone, Debug)]
struct SquareTime {
    pub start_time: usize,
    pub time_len: usize,
}

