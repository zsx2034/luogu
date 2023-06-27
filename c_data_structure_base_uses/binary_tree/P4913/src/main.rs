fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut tree: Vec<Option<Node>> = vec![None; n+1];

    for id in 1..=n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let left: usize = iter.next().unwrap().parse().unwrap();
        let right: usize = iter.next().unwrap().parse().unwrap();

        tree[id] = Some(Node {
            id,
            left: if left == 0 { None } else { Some(left) },
            right: if right == 0 { None } else { Some(right) },
        });
    }

    let h = depth(&tree, 1);

    println!("{h}");
}

fn depth(tree: &Vec<Option<Node>>, id: usize) -> usize {
    if tree[id].is_none() {
        return 0;
    }
    let node = tree[id].unwrap();
    let left = node.left.map(|id| depth(tree, id)).unwrap_or(0);
    let right = node.right.map(|id| depth(tree, id)).unwrap_or(0);
    std::cmp::max(left, right) + 1
}

#[derive(Clone, Copy)]
struct Node {
    id: usize,
    left: Option<usize>,
    right: Option<usize>,
}
