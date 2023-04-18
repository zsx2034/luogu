use std::borrow::BorrowMut;

fn main() {
    solution2();
}

// 线段树
fn solution2() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut input = buf.split_whitespace();
    let length_of_street = input.next().unwrap().parse::<usize>().unwrap();
    let count_of_areas = input.next().unwrap().parse::<usize>().unwrap();
    let mut seg_tree = SegmentTree::new(length_of_street, 1);
    for _ in 0..count_of_areas {
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("msg");
        input = buf.split_whitespace();
        let l = input.next().unwrap().parse::<usize>().unwrap();
        let r = input.next().unwrap().parse::<usize>().unwrap();
        seg_tree.revise_in_range(l, r, 0);
    }

    println!("{}", seg_tree.query_in_range(0, length_of_street, 0));
}

#[derive(Debug)]
struct SegmentTree {
    data: Vec<Node>,
}

#[derive(Clone, Copy, Debug)]
struct Node {
    value: i64,
    lazy: i64,
    lowbound: usize,
    upbound: usize,
}

impl Node {
    pub fn new(value: i64) -> Node {
        Node {
            value,
            lazy: 0,
            lowbound: 0,
            upbound: 0,
        }
    }

    pub fn is_point(self) -> bool {
        self.lowbound == self.upbound
    }
}

impl SegmentTree {
    pub fn new(n: usize, value: i64) -> SegmentTree {
        let data = vec![Node::new(value); n * 4];
        let mut tree = SegmentTree { data };
        tree.construct(0, 0, n);
        return tree;
    }

    pub fn construct(&mut self, pos: usize, lowbound: usize, upbound: usize) {
        self.data[pos].lowbound = lowbound;
        self.data[pos].upbound = upbound;
        if lowbound == upbound {
            return;
        }

        let mid = (lowbound + upbound) / 2;
        self.construct(Self::left(pos), lowbound, mid);
        self.construct(Self::right(pos), mid + 1, upbound);

        self.update(pos);
    }

    pub fn update(&mut self, pos: usize) {
        self.data[pos].value = self.data[Self::left(pos)].value + self.data[Self::right(pos)].value;
    }

    // 此题需要注意，因为每个叶子结点的最大值为1，最小值为0
    // 所以需要避免区间减法的过程中出现负值的情况 
    pub fn update_node_with_lazy(node: &mut Node) {
        let tmp = (node.upbound - node.lowbound + 1) as i64;
        node.value = if node.value < tmp { 0 } else { node.value - tmp };
    }

    pub fn query_in_range(&mut self, l: usize, r: usize, pos: usize) -> i64 {
        let node = self.data[pos].borrow_mut();

        // 当前区间为目标区间的子集
        if Self::comtains(l, r, node.lowbound, node.upbound) {
            return node.value;
        }

        // 当前区间不包含目标区间的子集
        if !Self::overlap(l, r, node.lowbound, node.upbound) {
            return 0;
        }

        // 判断是否需要lazy下传
        if node.lazy == -1 {
            self.lazy_down(pos);
        }

        let left_sum = self.query_in_range(l, r, Self::left(pos));
        let right_sum = self.query_in_range(l, r, Self::right(pos));
        return left_sum + right_sum;
    }

    pub fn revise_in_range(&mut self, l: usize, r: usize, pos: usize) {
        let node = self.data[pos].borrow_mut();

        // 当前区间为目标区间的子集
        if Self::comtains(l, r, node.lowbound, node.upbound) {
            if node.lazy != -1 {
                node.lazy = -1;
                Self::update_node_with_lazy(node);
            }
            return;
        }

        // 当前区间包含目标区间的子集
        if !Self::overlap(l, r, node.lowbound, node.upbound) {
            return;
        }

        // 判断是否需要lazy下传
        if node.lazy == -1 {
            self.lazy_down(pos);
        }

        self.revise_in_range(l, r, Self::left(pos));
        self.revise_in_range(l, r, Self::right(pos));
        self.update(pos);
    }

    pub fn lazy_down(&mut self, pos: usize) {
        self.data[pos].lazy = 0;
        // 更新左子树
        let left_child = self.data[Self::left(pos)].borrow_mut();
        left_child.lazy = -1;
        Self::update_node_with_lazy(left_child);

        let right_child = self.data[Self::right(pos)].borrow_mut();
        right_child.lazy = -1;
        Self::update_node_with_lazy(right_child);
    }

    // 判断a包含b，当a包含b时，返回true，否则，返回false
    pub fn comtains(la: usize, ra: usize, lb: usize, rb: usize) -> bool {
        la <= lb && ra >= rb
    }

    // // 返回a和b的重叠区域，当不存在重叠区域时，返回none
    // pub fn overlap(la: usize, ra: usize, lb: usize, rb: usize) -> Option<(usize, usize)> {
    //     //     la       ra
    //     // lb      rb
    //     if lb <= la && rb >= la {
    //         return Some((la, rb));
    //     }
    //     //  la           ra
    //     //         lb            rb
    //     else if lb <= ra && rb >= ra {
    //         return Some((lb, ra));
    //     }
    //     //        la  ra
    //     //     lb         rb
    //     else if lb <= la && rb >= ra {
    //         return Some((la, ra));
    //     }
    //     //     la           ra
    //     //          lb  rb
    //     else if lb >= la && rb <= ra {
    //         return Some((lb, rb));
    //     } else {
    //         return None;
    //     }
    // }

    pub fn overlap(la: usize, ra: usize, lb: usize, rb: usize) -> bool {
        return !(lb > ra || rb < la)
    }

    pub fn left(pos: usize) -> usize {
        pos * 2 + 1
    }

    pub fn right(pos: usize) -> usize {
        pos * 2 + 2
    }
}

// 模拟方式
fn solution1() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("ms");
    let mut input = buf.split_whitespace();
    let n: i64 = input.next().unwrap().parse().unwrap();
    let mut tree = vec![true; n as usize + 1];
    let area_number: i64 = input.next().unwrap().parse().unwrap();
    for _ in 0..area_number {
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("ms");
        input = buf.split_whitespace();
        let a = input.next().unwrap().parse::<usize>().unwrap();
        let b = input.next().unwrap().parse::<usize>().unwrap();
        for i in a..b + 1 {
            tree[i] = false;
        }
    }

    let mut sum = 0;
    for i in 0..n as usize + 1 {
        if tree[i] {
            sum += 1;
        }
    }
    println!("{}", sum);
}
