use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    parent: Option<Weak<RefCell<TreeNode<T>>>>,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Rc<RefCell<TreeNode<T>>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            parent: None,
            children: Vec::new(),
        }))
    }
}

fn main() {
    let root = TreeNode::new(1);
    let child = TreeNode::new(2);

    root.borrow_mut().children.push(Rc::clone(&child));
    child.borrow_mut().parent = Some(Rc::downgrade(&root));

    if let Some(parent_week) = &child.borrow().parent {
        if let Some(parent) = parent_week.upgrade() {
            println!("parent value: {}", parent.borrow().value);
        }
    }
}
