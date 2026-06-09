use std::{marker::PhantomData, ptr::NonNull};

/// raw pointer 기반 양방향 연결 리스트 노드
struct Node<T> {
    data: T,
    /// Some(_)인 경우에는 유효한 포인터가 들어가야 한다.
    next: Option<NonNull<Node<T>>>,
    /// Some(_)인 경우에는 유효한 포인터가 들어가야 한다.
    prev: Option<NonNull<Node<T>>>,
}

/// 양방향 연결 리스트
pub struct LinkedList<T> {
    /// Some(_)인 경우에는 유효한 포인터가 들어가야 한다.
    head: Option<NonNull<Node<T>>>,
    /// Some(_)인 경우에는 유효한 포인터가 들어가야 한다.
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> Node<T> {
    /// 힙에 노드를 할당하고 NonNull 포인터를 반환한다
    fn into_raw(data: T) -> NonNull<Self> {
        let boxed = Box::new(Node {
            data,
            next: None,
            prev: None,
        });

        // SAFETY: 나중에 반드시 해제해야 한다
        unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 리스트 뒤에 추가한다
    pub fn push_back(&mut self, data: T) {
        let mut new_node = Node::into_raw(data);

        match self.tail {
            None => {
                self.head = Some(new_node);
                self.tail = Some(new_node);
            }
            Some(mut old_tail) => unsafe {
                // SAFETY: old_tail은 유효한 포인터
                old_tail.as_mut().next = Some(new_node);
                new_node.as_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            },
        }

        self.len += 1;
    }

    /// 리스트 앞에 추가한다
    pub fn push_front(&mut self, data: T) {
        let mut new_node = Node::into_raw(data);

        match self.head {
            None => {
                self.head = Some(new_node);
                self.tail = Some(new_node);
            }
            Some(mut old_head) => unsafe {
                // SAFETY: old_head은 유효한 포인터
                old_head.as_mut().prev = Some(new_node);
                new_node.as_mut().next = Some(old_head);
                self.head = Some(new_node);
            },
        }

        self.len += 1;
    }

    /// 뒤에서 꺼낸다
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|old_tail_ptr| unsafe {
            // SAFETY: old_tail_ptr은 유효한 포인터

            let old_tail = old_tail_ptr.as_ptr();
            self.tail = (*old_tail).prev;
            match self.tail {
                None => self.head = None,
                Some(mut new_tail) => new_tail.as_mut().next = None,
            }
            self.len -= 1;

            Box::from_raw(old_tail).data
        })
    }

    /// 앞에서 꺼낸다
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|old_head_ptr| unsafe {
            // SAFETY: old_head_ptr는 유효한 포인터
            let old_head = old_head_ptr.as_ptr();
            self.head = (*old_head).next;
            match self.head {
                None => self.tail = None,
                Some(mut new_head) => new_head.as_mut().prev = None,
            }
            self.len -= 1;

            Box::from_raw(old_head).data
        })
    }

    /// 앞 요소 참조
    pub fn peek_front(&self) -> Option<&T> {
        self.head.map(|ptr| unsafe {
            // SAFETY: ptr은 유효한 포인터
            &ptr.as_ref().data
        })
    }

    /// 뒷 요소 참조
    pub fn peek_back(&self) -> Option<&T> {
        self.tail.map(|ptr| unsafe {
            // SAFETY: ptr은 유효한 포인터
            &ptr.as_ref().data
        })
    }

    /// 앞 요소 가변 참조
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.map(|mut ptr| unsafe {
            // SAFETY: ptr은 유효한 포인터
            &mut ptr.as_mut().data
        })
    }
}

/// 불변 이터레이터
pub struct Iter<'a, T> {
    front: Option<NonNull<Node<T>>>,
    back: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.head,
            back: self.tail,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.front.map(|ptr| unsafe {
            // SAFETY: ptr은 유효한 포인터
            let node = ptr.as_ptr();
            self.front = node.read().next;
            self.len -= 1;

            &ptr.as_ref().data
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        self.back.map(|ptr| unsafe {
            // SAFETY: ptr은 유효한 포인터
            let node = ptr.as_ptr();
            self.back = node.read().prev;
            self.len -= 1;

            &ptr.as_ref().data
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn push_pop() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn iter_forward_backward() {
        let mut list = LinkedList::new();

        (0..=5).for_each(|i| list.push_back(i));
        let fwd = list.iter().collect::<Vec<_>>();
        let bwd = list.iter().rev().collect::<Vec<_>>();

        assert_eq!(fwd, [&0, &1, &2, &3, &4, &5]);
        assert_eq!(bwd, [&5, &4, &3, &2, &1, &0]);
    }

    #[test]
    fn drop_does_not_leak() {
        let mut list = LinkedList::new();
        (0..1000).for_each(|i| list.push_back(format!("item - {}", i)));
    }
}
