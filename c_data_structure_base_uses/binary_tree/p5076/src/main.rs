use std::collections::VecDeque;

fn main() {
    let mut problem = Problem::new();
    problem.resolve();
}

struct Problem {
    n: usize,
    avl_tree: AVLTree,
}

impl Problem {
    pub fn new() -> Self {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        let avl_tree = AVLTree::new();

        Self { n, avl_tree }
    }

    fn resolve(&mut self) {
        for _ in 0..self.n {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let mut iter = buf.trim().split_whitespace();
            let op = iter.next().unwrap();
            let value: i32 = iter.next().unwrap().parse().unwrap();

            match op {
                "1" => {
                    let r = self.avl_tree.rank(value);
                    println!("{}", r + 1);
                }
                "2" => {
                    let num = self.avl_tree.search_n(value as usize);
                    println!("{}", num);
                }
                "3" => {
                    let num = self.avl_tree.front_node(value);
                    println!("{}", num);
                }
                "4" => {
                    let num = self.avl_tree.back_node(value);
                    println!("{}", num);
                }
                "5" => {
                    self.avl_tree.insert(value);
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    lchild: Option<Box<Node>>,
    rchild: Option<Box<Node>>,
    h: i32,
    total_cnt: usize,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            lchild: None,
            rchild: None,
            h: 1,
            total_cnt: 1,
        }
    }
}

struct AVLTree {
    root: Option<Box<Node>>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        AVLTree::_insert(&mut self.root, value);
        // println!("insert {} height {}", value, self.root.as_ref().unwrap().h);
        // dbg!(self.root.as_ref());
    }

    fn search(&self, value: i32) -> bool {
        if self.root.is_none() {
            return false;
        }
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => {
                    node = n.lchild.as_ref();
                }
                std::cmp::Ordering::Greater => {
                    node = n.rchild.as_ref();
                }
                std::cmp::Ordering::Equal => {
                    return true;
                }
            }
        }
        false
    }

    fn rank(&self, value: i32) -> usize {
        AVLTree::_rank(&self.root, value)
    }

    // query the rank of value
    fn _rank(root: &Option<Box<Node>>, value: i32) -> usize {
        if root.is_none() {
            return 0;
        }
        let mut node = root.as_ref();
        let mut rank = 0;
        while let Some(n) = node {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => {
                    node = n.lchild.as_ref();
                }
                std::cmp::Ordering::Greater => {
                    rank += AVLTree::_get_total_cnt(&n.lchild) + 1;
                    node = n.rchild.as_ref();
                }
                std::cmp::Ordering::Equal => {
                    let tmp = AVLTree::_rank(&n.lchild, value);
                    rank += tmp;
                    return rank;
                }
            }
        }
        rank
    }

    fn search_n(&self, n: usize) -> i32 {
        if self.root.is_none() {
            return i32::MAX;
        }
        let mut n = n;
        if n > self.root.as_ref().unwrap().total_cnt {
            return i32::MAX;
        }
        if let Some(res) = AVLTree::_search_n(self.root.as_ref().unwrap(), &mut n) {
            return res;
        } else {
            return i32::MAX;
        }
    }

    fn _search_n(node: &Box<Node>, n: &mut usize) -> Option<i32> {
        if node.lchild.is_some() && node.lchild.as_ref().unwrap().total_cnt >= *n {
            return AVLTree::_search_n(node.lchild.as_ref().unwrap(), n);
        }
        *n -= AVLTree::_get_total_cnt(&node.lchild) + 1;
        if *n == 0 {
            return Some(node.value);
        }
        if node.rchild.is_some() && node.rchild.as_ref().unwrap().total_cnt >= *n {
            return AVLTree::_search_n(node.rchild.as_ref().unwrap(), n);
        }
        *n -= AVLTree::_get_total_cnt(&node.rchild);
        None
    }

    fn back_node(&self, value: i32) -> i32 {
        // search for the back node of value
        if self.root.is_none() {
            return i32::MAX;
        }
        let mut node = self.root.as_ref();
        let mut back_node = i32::MAX;
        while let Some(n) = node {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => {
                    back_node = n.value;
                    node = n.lchild.as_ref();
                }
                std::cmp::Ordering::Greater => {
                    node = n.rchild.as_ref();
                }
                std::cmp::Ordering::Equal => {
                    if let Some(num) = AVLTree::_search_for_min(&n.rchild, value) {
                        return num;
                    }
                    return back_node;
                }
            }
        }
        back_node
    }

    fn front_node(&self, value: i32) -> i32 {
        // search for the front node of value
        if self.root.is_none() {
            return -2147483647;
        }
        let mut node = self.root.as_ref();
        let mut front_node = -2147483647;
        while let Some(n) = node {
            match value.cmp(&n.value) {
                std::cmp::Ordering::Less => {
                    node = n.lchild.as_ref();
                }
                std::cmp::Ordering::Greater => {
                    front_node = n.value;
                    node = n.rchild.as_ref();
                }
                std::cmp::Ordering::Equal => {
                    if let Some(num) = AVLTree::_search_for_max(&n.lchild, value) {
                        return num;
                    }
                    return front_node;
                }
            }
        }
        front_node
    }

    fn _insert(root: &mut Option<Box<Node>>, value: i32) {
        if root.is_none() {
            *root = Some(Box::new(Node::new(value)));
            return;
        }
        match value.cmp(&root.as_ref().unwrap().value) {
            std::cmp::Ordering::Less => {
                AVLTree::_insert(&mut root.as_mut().unwrap().lchild, value);
            }
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                AVLTree::_insert(&mut root.as_mut().unwrap().rchild, value);
            }
        }

        AVLTree::_update_height(root);
        AVLTree::_update_total_cnt(root);
        AVLTree::_rebalance(root);
    }

    fn _get_height(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(node) => node.h,
            None => 0,
        }
    }

    fn _height_diff(l: &Option<Box<Node>>, r: &Option<Box<Node>>) -> i32 {
        AVLTree::_get_height(l) - AVLTree::_get_height(r)
    }

    fn _right_rotate(root: &mut Option<Box<Node>>) {
        let mut new_root = root.as_mut().unwrap().lchild.take();
        root.as_mut().unwrap().lchild = new_root.as_mut().unwrap().rchild.take();
        new_root.as_mut().unwrap().rchild = root.take();
        *root = new_root;

        AVLTree::_update_height(&mut root.as_mut().unwrap().rchild);
        AVLTree::_update_height(root);
        AVLTree::_update_total_cnt(&mut root.as_mut().unwrap().rchild);
        AVLTree::_update_total_cnt(root);
    }

    fn _left_rotate(root: &mut Option<Box<Node>>) {
        let mut new_root = root.as_mut().unwrap().rchild.take();
        root.as_mut().unwrap().rchild = new_root.as_mut().unwrap().lchild.take();
        new_root.as_mut().unwrap().lchild = root.take();
        *root = new_root;

        AVLTree::_update_height(&mut root.as_mut().unwrap().lchild);
        AVLTree::_update_height(root);
        AVLTree::_update_total_cnt(&mut root.as_mut().unwrap().lchild);
        AVLTree::_update_total_cnt(root);
    }

    fn _rebalance(root: &mut Option<Box<Node>>) {
        let height_diff = AVLTree::_height_diff(
            &root.as_ref().unwrap().lchild,
            &root.as_ref().unwrap().rchild,
        );
        if height_diff > 1 {
            if AVLTree::_height_diff(
                &root.as_ref().unwrap().lchild.as_ref().unwrap().lchild,
                &root.as_ref().unwrap().lchild.as_ref().unwrap().rchild,
            ) > 0
            {
                AVLTree::_right_rotate(root);
            } else {
                AVLTree::_left_rotate(&mut root.as_mut().unwrap().lchild);
                AVLTree::_right_rotate(root);
            }
        } else if height_diff < -1 {
            if AVLTree::_height_diff(
                &root.as_ref().unwrap().rchild.as_ref().unwrap().lchild,
                &root.as_ref().unwrap().rchild.as_ref().unwrap().rchild,
            ) < 0
            {
                AVLTree::_left_rotate(root);
            } else {
                AVLTree::_right_rotate(&mut root.as_mut().unwrap().rchild);
                AVLTree::_left_rotate(root);
            }
        }
    }

    fn _update_height(node: &mut Option<Box<Node>>) {

        if node.is_none() {
            return;
        }

        node.as_mut().unwrap().h = std::cmp::max(
            AVLTree::_get_height(&node.as_ref().unwrap().lchild),
            AVLTree::_get_height(&node.as_ref().unwrap().rchild),
        ) + 1;
    }

    fn _get_total_cnt(node: &Option<Box<Node>>) -> usize {
        if node.is_none() {
            return 0;
        }
        node.as_ref().unwrap().total_cnt
    }

    fn _update_total_cnt(node: &mut Option<Box<Node>>) {
        // dbg!(&node);

        if node.is_none() {
            return;
        }
        node.as_mut().unwrap().total_cnt = AVLTree::_get_total_cnt(&node.as_ref().unwrap().lchild)
            + AVLTree::_get_total_cnt(&node.as_ref().unwrap().rchild)
            + 1;
    }

    fn display(&self) {
        let mut q1 = VecDeque::new();
        let mut q2 = VecDeque::new();

        if self.root.is_none() {
            println!("*");
            return;
        }
        q1.push_back(self.root.as_ref());
        while !q1.is_empty() {
            let mut line = String::new();
            while !q1.is_empty() {
                let node = q1.pop_front().unwrap();
                match node {
                    Some(node) => {
                        line.push_str(&node.value.to_string());
                        line.push(' ');
                        q2.push_back(node.lchild.as_ref());
                        q2.push_back(node.rchild.as_ref());
                    }
                    None => {
                        line.push_str("* ");
                    }
                }
            }
            println!("{}", line);
            std::mem::swap(&mut q1, &mut q2);

            let mut flag = false;
            for node in q1.iter() {
                if node.is_some() {
                    flag = true;
                    break;
                }
            }

            if !flag {
                break;
            }
        }
    }

    fn _search_for_min(node: &Option<Box<Node>>, value: i32) -> Option<i32> {
        // search for the min value that is greater than value
        if node.is_none() {
            return None;
        }
        if node.as_ref().unwrap().value > value {
            let min = AVLTree::_search_for_min(&node.as_ref().unwrap().lchild, value);
            if min.is_some() {
                return min;
            }
            return Some(node.as_ref().unwrap().value);
        }

        AVLTree::_search_for_min(&node.as_ref().unwrap().rchild, value)
    }

    fn _search_for_max(node: &Option<Box<Node>>, value: i32) -> Option<i32> {
        // search for the max value that is less than value
        if node.is_none() {
            return None;
        }
        if node.as_ref().unwrap().value < value {
            let max = AVLTree::_search_for_max(&node.as_ref().unwrap().rchild, value);
            if max.is_some() {
                return max;
            }
            return Some(node.as_ref().unwrap().value);
        }
        AVLTree::_search_for_max(&node.as_ref().unwrap().lchild, value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_back_node() {
        let mut avl = AVLTree::new();
        // avl.insert(1);
        // avl.insert(2);
        // avl.insert(2);
        // avl.insert(3);
        // avl.insert(3);
        // avl.insert(3);
        // avl.insert(4);
        // avl.insert(3);
        // avl.insert(3);
        // avl.insert(5);
        // avl.insert(6);

        avl.insert(2);
        avl.insert(4);
        avl.insert(4);
        avl.insert(6);
        avl.insert(6);
        avl.insert(6);
        avl.insert(8);
        avl.insert(6);
        avl.insert(6);
        avl.insert(10);
        avl.insert(12);
    
        avl.display();
    
        // assert_eq!(avl.back_node(1), 2);
        // assert_eq!(avl.back_node(2), 3);
        // assert_eq!(avl.back_node(3), 4);
        // assert_eq!(avl.back_node(4), 5);
        // assert_eq!(avl.back_node(5), 6);
        // assert_eq!(avl.back_node(6), i32::MAX);

        assert_eq!(avl.back_node(1), 2);
        assert_eq!(avl.back_node(2), 4);
        assert_eq!(avl.back_node(3), 4);
        assert_eq!(avl.back_node(4), 6);
        assert_eq!(avl.back_node(5), 6);
        assert_eq!(avl.back_node(6), 8);
        assert_eq!(avl.back_node(7), 8);
        assert_eq!(avl.back_node(8), 10);
        assert_eq!(avl.back_node(9), 10);
        assert_eq!(avl.back_node(10), 12);
        assert_eq!(avl.back_node(11), 12);
        assert_eq!(avl.back_node(12), i32::MAX);
        assert_eq!(avl.back_node(13), i32::MAX);
    }

    #[test]
    fn test_front_node() {
        let mut avl = AVLTree::new();
        // avl.insert(1);
        // avl.insert(2);
        // avl.insert(2);
        // avl.insert(2);
        // avl.insert(2);
        // avl.insert(2);
        // avl.insert(4);
        // avl.insert(3);
        // avl.insert(3);
        // avl.insert(5);
        // avl.insert(6);

        avl.insert(2);
        avl.insert(4);
        avl.insert(4);
        avl.insert(4);
        avl.insert(4);
        avl.insert(4);
        avl.insert(8);
        avl.insert(6);
        avl.insert(6);
        avl.insert(10);
        avl.insert(12);

        // assert_eq!(avl.front_node(1), i32::MIN + 1);
        // assert_eq!(avl.front_node(2), 1);
        // assert_eq!(avl.front_node(3), 2);
        // assert_eq!(avl.front_node(4), 3);
        // assert_eq!(avl.front_node(5), 4);
        // assert_eq!(avl.front_node(6), 5);

        assert_eq!(avl.front_node(1), i32::MIN + 1);
        assert_eq!(avl.front_node(2), i32::MIN + 1);
        assert_eq!(avl.front_node(3), 2);
        assert_eq!(avl.front_node(4), 2);
        assert_eq!(avl.front_node(5), 4);
        assert_eq!(avl.front_node(6), 4);
        assert_eq!(avl.front_node(7), 6);
        assert_eq!(avl.front_node(8), 6);
        assert_eq!(avl.front_node(9), 8);
        assert_eq!(avl.front_node(10), 8);
        assert_eq!(avl.front_node(11), 10);
        assert_eq!(avl.front_node(12), 10);
        assert_eq!(avl.front_node(13), 12);
        assert_eq!(avl.front_node(14), 12);
    }

    #[test]
    fn test_rank() {
        let mut avl = AVLTree::new();
        avl.insert(1);
        avl.insert(2);
        avl.insert(2);
        avl.insert(2);
        avl.insert(2);
        avl.insert(2);
        avl.insert(3);
        avl.insert(3);
        avl.insert(4);
        avl.insert(5);
        avl.insert(6);

        avl.display();

        assert_eq!(avl.rank(1) + 1, 1);
        assert_eq!(avl.rank(2) + 1, 2);
        assert_eq!(avl.rank(3) + 1, 7);
        assert_eq!(avl.rank(4) + 1, 9);
        assert_eq!(avl.rank(5) + 1, 10);
        assert_eq!(avl.rank(6) + 1, 11);
    }

    #[test]
    fn test_search_n() {
        let mut avl = AVLTree::new();
        avl.insert(1);
        avl.insert(2);
        avl.insert(2);
        avl.insert(2);
        avl.insert(2);
        avl.insert(2);
        avl.insert(3);
        avl.insert(3);
        avl.insert(4);
        avl.insert(5);
        avl.insert(6);

        avl.display();

        assert_eq!(avl.search_n(1), 1);
        assert_eq!(avl.search_n(2), 2);
        assert_eq!(avl.search_n(3), 2);
        assert_eq!(avl.search_n(4), 2);
        assert_eq!(avl.search_n(5), 2);
        assert_eq!(avl.search_n(6), 2);
        assert_eq!(avl.search_n(7), 3);
        assert_eq!(avl.search_n(8), 3);
        assert_eq!(avl.search_n(9), 4);
        assert_eq!(avl.search_n(10), 5);
        assert_eq!(avl.search_n(11), 6);
        assert_eq!(avl.search_n(12), i32::MAX);
    }
}
