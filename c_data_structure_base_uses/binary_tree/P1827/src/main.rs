fn main() {
    let mut problem = Problem::new();
    problem.build();
    problem.post_order();
}

struct Node {
    v: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(v: char) -> Self {
        Node {
            v,
            left: None,
            right: None,
        }
    }
}

struct Problem {
    pre_order: Vec<char>,
    mid_order: Vec<char>,
    head: Option<Box<Node>>,
}

impl Problem {
    fn new() -> Problem {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mid_order: Vec<char> = buf.trim().chars().collect();
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let pre_order: Vec<char> = buf.trim().chars().collect();
        Problem {
            pre_order,
            mid_order,
            head: None,
        }
    }    

    fn build(&mut self) {
        self.head = Some(Box::new(self.build_tree(0, (0, self.mid_order.len() - 1))));
    }

    fn build_tree(&mut self, pre_index: usize, mid_range: (usize, usize)) -> Node {
        let v = self.pre_order[pre_index];
        let mut node = Node::new(v);
        let mid_order_index: usize = {
            let mut res = mid_range.1 + 1;
            for i in mid_range.0..=mid_range.1 {
                if self.mid_order[i] == v {
                    res = i;
                    break;
                }
            }
            res
        };
        if mid_range.0 < mid_order_index && mid_order_index <= mid_range.1{ 
            node.left = Some(Box::new(self.build_tree(pre_index + 1, (mid_range.0, mid_order_index - 1))));
        }
        if mid_order_index < mid_range.1 && mid_order_index >= mid_range.0 {
            node.right = Some(Box::new(self.build_tree(pre_index + mid_order_index + 1 - mid_range.0, (mid_order_index + 1, mid_range.1))));
        }
        node
    }

    fn post_order(&self) {
        let mut buf = String::new();
        self.post_order_tree(&self.head, &mut buf);
        println!("{}", buf.trim());
    }

    fn post_order_tree(&self, node: &Option<Box<Node>>, buf: &mut String) {
        if let Some(node) = node {
            self.post_order_tree(&node.left, buf);
            self.post_order_tree(&node.right, buf);
            buf.push(node.v);
        }
    }
}

