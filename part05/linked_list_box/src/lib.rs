/// Box 기반 단방향 연결 리스트
pub enum List<T> {
    /// 빈 리스트 또는 끝
    Nil,
    /// 데이터와 다음 노드
    Cons(T, Box<List<T>>),
}

/// 단방향 연결 리스트
pub struct LinkedList<T> {
    head: List<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: List::Nil,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 리스트 앞에 요소를 추가한다.
    pub fn push_front(&mut self, data: T) {
        let old_head = std::mem::replace(&mut self.head, List::Nil);
        self.head = List::Cons(data, Box::new(old_head));
        self.len += 1;
    }

    /// 리스트 앞에서 요소를 꺼낸다.
    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = std::mem::replace(&mut self.head, List::Nil);
        match old_head {
            List::Nil => None,
            List::Cons(data, next) => {
                self.head = *next;
                self.len -= 1;
                Some(data)
            }
        }
    }

    /// 앞 요소를 참조한다.
    pub fn peek_front(&self) -> Option<&T> {
        match &self.head {
            List::Nil => None,
            List::Cons(data, _) => Some(data),
        }
    }

    /// 앞 요소를 가변 참조한다.
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        match &mut self.head {
            List::Nil => None,
            List::Cons(data, _) => Some(data),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
}

/// 불변 순회 이터레이터
pub struct Iter<'a, T> {
    next: &'a List<T>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: &self.head }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            List::Nil => None,
            List::Cons(data, next) => {
                self.next = next;
                Some(data)
            }
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

/// 소비 이터레이터
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
