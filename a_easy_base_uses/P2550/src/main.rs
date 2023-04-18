fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i32>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut template = vec![0;50];
    buf.split_whitespace().for_each(|s| {
        template[s.trim().parse::<usize>().unwrap()] = 1;
    });

    let mut res: Vec<i32> = vec![0; 8];

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("msg");
        let nums = buf
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        
        let mut cnt = 0;
        for num in nums {
            if template[num as usize] == 1 {
                cnt += 1;
            }
        }

        res[7 - cnt] += 1;
    }

    (0..7).for_each(|i| {
        print!("{} ", res[i]);
    });
}

fn solution1() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i32>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut template = buf
        .split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    template.sort();

    let mut res: Vec<i32> = vec![0; 8];

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("msg");
        let mut nums = buf
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        nums.sort();
        res[check1(&nums, &template)] += 1;
    }

    (0..7).for_each(|i| {
        print!("{} ", res[i]);
    });
}

fn check1(nums: &Vec<i32>, template: &Vec<i32>) -> usize {
    let mut champ: usize = 0;
    let mut i = 0;
    let mut j = 0;
    while i < 7 && j < 7 {
        match nums[i].cmp(&template[j]) {
            std::cmp::Ordering::Less => {
                i += 1;
            }
            std::cmp::Ordering::Equal => {
                champ += 1;
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => {
                j += 1;
            }
        }
    }
    return 7 - champ;
}
