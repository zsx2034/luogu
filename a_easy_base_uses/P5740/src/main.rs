fn main() {
    solution2();
}

fn solution1() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let mut students = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let stu: Student = buf.split_whitespace().collect();
        students.push(stu);
    }
    students.sort_by(|a, b| {
        b.sum.cmp(&a.sum)
    });

    dbg!(&students);

    print!("{} {} {} {}", students[0].name, students[0].chinese, students[0].math, students[0].english);
}

#[derive(Debug)]
struct Student {
    name: String,
    chinese: u16,
    math: u16,
    english: u16,
    sum: u16
}

impl<'a>  FromIterator<&'a str> for Student {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let name = iter.next().unwrap().to_string();
        let chinese = iter.next().unwrap().parse::<u16>().unwrap();
        let math = iter.next().unwrap().parse::<u16>().unwrap();
        let english = iter.next().unwrap().parse::<u16>().unwrap();
        Student { name, chinese, math, english, sum: chinese + math + english }
    }
}


fn solution2() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let mut name = String::new();
    let mut chinese = 0;
    let mut math = 0;
    let mut english = 0;
    let mut max = 0;
    let mut flag = true;
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let name_ = iter.next().unwrap().to_string();
        let chinese_ = iter.next().unwrap().parse::<u16>().unwrap();
        let math_ = iter.next().unwrap().parse::<u16>().unwrap();
        let english_ = iter.next().unwrap().parse::<u16>().unwrap();
        let sum = chinese_ + math_ + english_;
        if sum > max || flag {
            name = name_;
            chinese = chinese_;
            math = math_;
            english = english_;
            max = sum;
            flag = false;
        }
    }

    print!("{} {} {} {}", name, chinese, math, english);
}