fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let robot_number: usize = iter.next().unwrap().parse().unwrap();
    let instruction_number: usize = iter.next().unwrap().parse().unwrap();

    let mut rebots_direction = Vec::with_capacity(robot_number);
    let mut rebots_name = Vec::with_capacity(robot_number);
    for _ in 0..robot_number {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let direction: i32 = iter.next().unwrap().parse().unwrap();
        let name = iter.next().unwrap().to_string();
        rebots_direction.push(direction);
        rebots_name.push(name);
    }

    let mut postion: usize = 0;
    for _ in 0..instruction_number {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let direction = iter.next().unwrap().parse::<i32>().unwrap();
        let step = iter.next().unwrap().parse::<usize>().unwrap();

        if (direction == 1 && rebots_direction[postion] == 1)
            || (direction == 0 && rebots_direction[postion] == 0)
        {
            postion = (postion + robot_number - step) % robot_number;
            // println!("current postion is: {}, and its name is {}", postion, rebots[postion as usize].name);
        } else {
            postion = (postion + step) % robot_number;
            // println!("current postion is: {}, and its name is {}", postion, rebots[postion as usize].name);
        }
    }

    println!("{}", rebots_name[postion]);
}