fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut persons = Vec::with_capacity(n);
    for id in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut p = buf.trim().split_whitespace().collect::<Person>();
        p.set_id(id);
        persons.push(p);
    }

    persons.sort_by(
        |a, b| {
            if a.year_of_birth != b.year_of_birth {
                a.year_of_birth.cmp(&b.year_of_birth)
            } else if a.month_of_birth != b.month_of_birth {
                a.month_of_birth.cmp(&b.month_of_birth)
            } else if a.day_of_birth != b.day_of_birth {
                a.day_of_birth.cmp(&b.day_of_birth)
            } else {
                b.id.cmp(&a.id)
            }
        }
    );

    for p in persons {
        println!("{}", p.name);
    }
}

struct Person {
    id: usize,
    name: String,
    year_of_birth: u32,
    month_of_birth: u32,
    day_of_birth: u32,
}

impl Person {
    fn new(name: String, year_of_birth: u32, month_of_birth: u32, day_of_birth: u32) -> Self {
        Self {
            id: 0,
            name,
            year_of_birth,
            month_of_birth,
            day_of_birth,
        }
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl<'a> FromIterator<&'a str> for Person {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let name = iter.next().unwrap().to_string();
        let year_of_birth = iter.next().unwrap().parse().unwrap();
        let month_of_birth = iter.next().unwrap().parse().unwrap();
        let day_of_birth = iter.next().unwrap().parse().unwrap();
        Person::new(name, year_of_birth, month_of_birth, day_of_birth)
    }
}