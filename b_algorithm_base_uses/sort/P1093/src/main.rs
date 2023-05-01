fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut students: Vec<Student> = Vec::with_capacity(n);
    for id in 1..n + 1 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut stu: Student = buf
            .split_whitespace()
            .collect();
        stu.set_id(id);
        students.push(stu);
    }

    students.sort_by(|a, b| {
        if a.total == b.total {
            if a.chinese == b.chinese {
                a.id.cmp(&b.id)
            } else {
                b.chinese.cmp(&a.chinese)
            }
        } else {
            b.total.cmp(&a.total)
        }
    } );

    for i in 0..5 {
        println!("{} {}", students[i].id, students[i].total);
    }
}

#[derive()]
struct Student {
    id: usize,
    chinese: usize,
    math: usize,
    english: usize,
    total: usize,
}

impl Student {
    fn new(chinese: usize, math: usize, english: usize) -> Self {
        let total = chinese + math + english;
        Self {
            id: 0,
            chinese,
            math,
            english,
            total,
        }
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl<'a> FromIterator<&'a str> for Student {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let chinese: usize = iter.next().unwrap().parse().unwrap();
        let math: usize = iter.next().unwrap().parse().unwrap();
        let english: usize = iter.next().unwrap().parse().unwrap();
        Self::new(chinese, math, english)
    }
}
