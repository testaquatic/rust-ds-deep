use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// 양방향 연결 리스트의 노드
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

/// 단방향 연결 리스트
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// 리스트 뒤에 노드를 추가한다
    pub fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
    }

    /// 리스트 앞에 노드를 추가한다
    pub fn push_front(&mut self, data: T) {
        let new_node = Node::new(data);
        match self.head.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 뒤에서 꺼낸다
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                None => {
                    self.head.take();
                }

                Some(prev_weak) => {
                    if let Some(prev_rc) = prev_weak.upgrade() {
                        prev_rc.borrow_mut().next = None;
                        self.tail = Some(prev_rc);
                    }
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_tail)
                .ok()
                .expect("RC 카운트가 1이 아닙니다")
                .into_inner()
                .data
        })
    }

    /// 얖에서 꺼낸다
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                None => {
                    self.tail.take();
                }

                Some(next_rc) => {
                    next_rc.borrow_mut().prev = None;
                    self.head = Some(next_rc);
                }
            }

            self.len -= 1;
            Rc::try_unwrap(old_head)
                .ok()
                .expect("RC 카운트가 1이 아닙니다")
                .into_inner()
                .data
        })
    }
}

impl<T: Clone> DoublyLinkedList<T> {
    /// 앞 요소를 복제해 반환한다
    pub fn peek_front(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().data.clone())
    }

    pub fn peek_back(&self) -> Option<T> {
        self.tail.as_ref().map(|node| node.borrow().data.clone())
    }

    /// 앞에서 뒤로 순회하며 Vec을 수집한다
    pub fn to_vec_forward(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        let mut current = self.head.clone();
        while let Some(node) = current {
            result.push(node.borrow().data.clone());
            current = node.borrow().next.clone();
        }
        result
    }

    /// 뒤에서 앞으로 순회하며 Vec을 수집한다
    pub fn to_vec_backward(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        let mut current = self.tail.clone();

        while let Some(node) = current {
            result.push(node.borrow().data.clone());
            current = node.borrow().prev.as_ref().and_then(|weak| weak.upgrade());
        }

        result
    }
}
