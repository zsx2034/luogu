use std::vec;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut students: Vec<Student> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let student: Student = buf.split_whitespace().collect();
        students.push(student);
    }

    for i in 0..n {
        for j in i+1..n {
            if students[i].equal(&students[j]) {
                println!("{} {}", students[i].name, students[j].name);
            }
        }
    }
}

struct Student {
    name: String,
    chinese: i32,
    math: i32,
    english: i32,
    sum: i32,
}

impl Student {
    fn new(name: String, chinese: i32, math: i32, english: i32) -> Self {
        Self {
            name,
            chinese,
            math,
            english,
            sum: chinese + math + english,
        }
    }

    fn equal(&self, other: &Self) -> bool {
        (self.chinese - other.chinese).abs() <= 5
            && (self.math - other.math).abs() <= 5
            && (self.english - other.english).abs() <= 5
            && (self.sum - other.sum).abs() <= 10
    }
}

impl<'a> FromIterator<&'a str> for Student {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let name = iter.next().unwrap().to_string();
        let chinese = iter.next().unwrap().parse().unwrap();
        let math = iter.next().unwrap().parse().unwrap();
        let english = iter.next().unwrap().parse().unwrap();
        Self::new(name, chinese, math, english)
    }
}
