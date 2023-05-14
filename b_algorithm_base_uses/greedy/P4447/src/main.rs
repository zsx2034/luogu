use std::collections::{HashMap, LinkedList, VecDeque};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    v.sort();
    // println!("{:?}", v);

    let mut map: HashMap<i32, IntervalSet> = HashMap::new();

    for i in 0..n {
        if let Some(set) = map.get_mut(&(v[i] - 1)) {
            let mut interval = set.pop_front().unwrap();
            if set.is_empty() {
                map.remove(&(v[i] - 1));
            }
            interval.extend();
            map.entry(v[i])
                .and_modify(|x| x.insert(interval))
                .or_insert({
                    let mut set = IntervalSet::new();
                    set.insert(interval);
                    set
                });
        } else {
            map.entry(v[i])
                .and_modify(|x| x.insert(Interval::new(v[i], v[i])))
                .or_insert({
                    let mut set = IntervalSet::new();
                    set.insert(Interval::new(v[i], v[i]));
                    set
                });
        }
    }

    let mut min = std::i32::MAX;
    for (k, v) in map.iter() {
        // println!("{}: {:?}", k, v);

        if v.min() < min {
            min = v.min();
        }
    }

    println!("{}", min);
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    start: i32,
    end: i32,
    len: i32,
}

impl Interval {
    fn new(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            len: end - start + 1,
        }
    }

    fn extend(&mut self) {
        self.end += 1;
        self.len += 1;
    }
}

#[derive(Debug)]
struct IntervalSet {
    intervals: VecDeque<Interval>,
}

impl IntervalSet {
    fn new() -> Self {
        Self {
            intervals: VecDeque::new(),
        }
    }

    fn insert(&mut self, interval: Interval) {
        for (index, item) in self.intervals.iter().enumerate() {
            if item.len > interval.len {
                self.intervals.insert(index, interval);
                return;
            }
        }
        self.intervals.push_back(interval);
    }

    fn pop_front(&mut self) -> Option<Interval> {
        self.intervals.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    fn min(&self) -> i32 {
        self.intervals.front().unwrap().len
    }
}
