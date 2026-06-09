use std::collections::VecDeque;

/// 이전 트리 노드
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Box<Self> {
        Box::new(TreeNode {
            val,
            left: None,
            right: None,
        })
    }
}

/// 레벨 순서 순회
fn level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();

        for _ in 0..level_size {
            let node = queue.pop_front().unwrap();
            level.push(node.val);
            if let Some(left) = node.left {
                queue.push_back(left);
            }
            if let Some(right) = node.right {
                queue.push_back(right);
            }
        }
        result.push(level);
    }
    result
}

fn main() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    left.left = Some(TreeNode::new(4));
    left.right = Some(TreeNode::new(5));
    root.left = Some(left);
    root.right = Some(TreeNode::new(3));

    let levels = level_order(Some(root));
    for (i, level) in levels.iter().enumerate() {
        println!("레벨 {}: {:?}", i, level);
    }
}
