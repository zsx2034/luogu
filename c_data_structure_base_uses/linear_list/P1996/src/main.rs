use std::{rc::Rc, borrow::{Borrow, BorrowMut}, cell::RefCell, ops::DerefMut};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    if n < 1 {
        return ;
    }

    let mut list = List::new();
    for i in 1..=n {
        list.push(i);
    }

    let mut node = list.get_head().unwrap();
    while list.get_len() > 0 {
        for _ in 0..m-1 {
            let next = (*node).borrow().get_next().unwrap();
            node = next;
        }
        let next = (*node).borrow().get_next().unwrap();
        list.remove(Rc::clone(&node));
        print!("{} ", (*node).borrow().get_id());
        node = next;
    }
}

struct Node {
    id: usize,
    next: Option<Rc<RefCell<Node>>>,
    last: Option<Rc<RefCell<Node>>>
}

impl Node {
    fn new(id: usize) -> Self {
        Node {
            id,
            next: None,
            last: None
        }
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Node>>) {
        self.next = Some(next);
    }

    pub fn set_last(&mut self, last: Rc<RefCell<Node>>) {
        self.last = Some(last);
    }

    pub fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
        self.next.clone()
    }

    pub fn get_last(&self) -> Option<Rc<RefCell<Node>>> {
        self.last.clone()
    }

    pub fn get_id(&self) -> &usize {
        &self.id
    }
}

struct List {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            len : 0
        }
    }

    fn push(&mut self, id: usize) {
        let node = Rc::new(RefCell::new(Node::new(id)));
        match self.tail {
            Some(ref mut tail) => {
                let tail = tail.clone();
                (*tail).borrow_mut().set_next(Rc::clone(&node));
                (*node).borrow_mut().set_last(Rc::clone(&tail));
                (*node).borrow_mut().set_next(Rc::clone(self.head.as_ref().unwrap()));
                let head = Rc::clone(self.head.as_ref().unwrap());
                (*head).borrow_mut().set_last(Rc::clone(&node));
                self.tail = Some(node);
            },
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
                (*node).borrow_mut().set_last(Rc::clone(&node));
                (*node).borrow_mut().set_next(Rc::clone(&node));
            }
        }
        self.len += 1;
    }

    fn display(&self) {
        if let Some(ref head) = self.head {
            let mut node = Rc::clone(head);
            for _ in 0..self.len {
                print!("{} ", (*node).borrow().get_id());
                let tmp = (*node).borrow().get_next();
                if let Some(next) = tmp {
                    node = next;
                }
            }
        }
    }

    fn get_head(&self) -> Option<Rc<RefCell<Node>>> {
        self.head.clone()
    }

    fn get_len(&self) -> usize {
        self.len
    }

    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        if self.len == 0 {
            return ;
        }

        if self.len == 1 {
            self.head = None;
            self.tail = None;
            self.len = 0;
            return ;
        }

        let last = (*node).borrow().get_last().unwrap();
        let next = (*node).borrow().get_next().unwrap();
        (*last).borrow_mut().set_next(Rc::clone(&next));
        (*next).borrow_mut().set_last(Rc::clone(&last));

        if Rc::ptr_eq(&node, self.head.as_ref().unwrap()) {
            self.head = Some(Rc::clone(&next));
        }
        if Rc::ptr_eq(&node, self.tail.as_ref().unwrap()) {
            self.tail = Some(Rc::clone(&last));
        }
        self.len -= 1;
    }
}


#[test]
fn test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);

    list.display();

}

