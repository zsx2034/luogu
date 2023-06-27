use std::{collections::VecDeque, borrow::Borrow};

fn main() {
    let mut p = Problem::new();
    let ans = p.solve();
    println!("{}", ans);
}

struct Problem {
    n: usize,
    v: Vec<Node>,
}

impl Problem {
    fn new() -> Self {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();
        
        let mut v = Vec::with_capacity(n);
        v.push(Node::new(None, None, 0));
        for _ in 0..n {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let mut iter = buf.trim().split_whitespace();
            let w: i32 = iter.next().unwrap().parse().unwrap();
            let lchild: Option<usize> = {
                let tmp: usize = iter.next().unwrap().parse().unwrap();
                if tmp == 0 {
                    None
                } else {
                    Some(tmp)
                }
            };
            let rchild: Option<usize> = {
                let tmp: usize = iter.next().unwrap().parse().unwrap();
                if tmp == 0 {
                    None
                } else {
                    Some(tmp)
                }
            };
    
            v.push(Node::new(lchild, rchild, w));
        }

        Problem {
            n,
            v,
        }
    }
    
    /// 经过预处理后计算每个节点的total
    /// 采用层序遍历
    /// total = 子树所有人到达当前节点的路径和（lt + rt） + 父节点所有人到达当前节点的路径和
    /// 同时需要自上而下更新每个节点的w
    ///     当前节点的 w = 非当前节点子树的节点的 w 和
    ///         非子树：所有经过父节点到达当前节点的人数和，包括父节点
    fn solve(&mut self) -> i32{
        self.pre_process(1, None);

        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(1);
        let mut ans = i32::MAX;
        while let Some(p) = q.pop_front() {
            let tmp = self.process_parent(p);
            self.v[p].total = tmp.1 + self.v[p].lt + self.v[p].rt;
            self.v[p].w += tmp.0;
            if let Some(l) = self.v[p].lchild {
                q.push_back(l);
            }
            if let Some(r) = self.v[p].rchild {
                q.push_back(r);
            }
            ans = ans.min(self.v[p].total);
        }

        return ans;
    }

    // 计算父节点的w和路径和
    #[inline]
    fn process_parent(&mut self, p: usize) -> (i32, i32) {
        if self.v[p].parent.is_none() {
            return (0, 0);
        }

        let parent = self.v[p].parent.unwrap();
        if self.v[parent].lchild.is_some() && self.v[parent].lchild.unwrap() == p {
            let t =  self.v[parent].total - self.v[parent].lt + self.v[parent].rw + self.v[parent].w;
            let w = self.v[parent].w + self.v[parent].rw;
            return (w, t);
        } else if self.v[parent].rchild.is_some() && self.v[parent].rchild.unwrap() == p {
            let t =  self.v[parent].total - self.v[parent].rt + self.v[parent].lw + self.v[parent].w;
            let w = self.v[parent].w + self.v[parent].lw;
            return (w, t);
        }

        (0, 0)
    }

    /// 预处理，计算每个节点的lw, lt, rw, rt
    /// 采用后序遍历
    /// lw: 左子树人数
    /// lt: 左子树所有人到达当前节点走过的路径和
    /// rw: 右子树人数
    /// rt: 右子树所有人到达当前节点走过的路径和
    /// 当前节点的lt = 左子树的所有人到达左子树的路径和 + 左子树的人数和
    /// 当前节点的rt = 右子树的所有人到达右子树的路径和 + 右子树的人数和
    fn pre_process(&mut self, p: usize, parent: Option<usize>) {
        let lchild = self.v[p].lchild;
        let rchild = self.v[p].rchild;
        if let Some(l) = lchild {
            self.pre_process(l, Some(p));
        }
        if let Some(r) = rchild {
            self.pre_process(r, Some(p));
        }
        let (lw, lt) = match lchild {
            Some(l) => {
                let lw = self.v[l].lw + self.v[l].w + self.v[l].rw;
                let lt = self.v[l].lt + self.v[l].rt + lw;
                (lw, lt)
            }
            None => (0, 0),
        };
        let (rw, rt) = match rchild {
            Some(r) => {
                let rw = self.v[r].lw + self.v[r].w + self.v[r].rw;
                let rt = self.v[r].lt + self.v[r].rt + rw;
                (rw, rt)
            }
            None => (0, 0),
        };
        self.v[p].parent = parent;
        self.v[p].lw = lw;
        self.v[p].lt = lt;
        self.v[p].rw = rw;
        self.v[p].rt = rt;
    }


}

struct Node {
    parent: Option<usize>,
    lchild: Option<usize>,
    rchild: Option<usize>,
    // 当前节点的值，可以理解为人数
    w: i32,
    // 左子树人数
    lw: i32,
    // 左子树所有人到达当前节点走过的路径和
    lt: i32,
    // 右子树人数
    rw: i32,
    // 右子树所有人到达当前节点走过的路径和
    rt: i32,
    // 树中所有人到达当前节点走过的路径和
    total: i32,
}

impl Node {
    fn new(lchild: Option<usize>, rchild: Option<usize>, w: i32) -> Self {
        Node {
            parent: None,
            lchild,
            rchild,
            w,
            lw: 0,
            rw: 0,
            lt: 0,
            rt: 0,
            total: 0,
        }
    }
}