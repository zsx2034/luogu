fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut nums = buf
        .split_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    nums.sort();
    if nums[2] - nums[1] + 1 >= nums[0] {
        println!("{}", 1 + nums[0] + nums[1]);
    } else {
        println!("{}", ((nums[1] + nums[2] + 2) as f32 / 2.0 + nums[0] as f32 / 2.0).round() as i32);
    }
}
