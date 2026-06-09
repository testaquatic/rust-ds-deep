#[derive(Debug)]
pub struct Node<T> {
    data: T,
    /// 다음 노드의 인덱스
    /// None이면 이 노드가 최종 노드
    next: Option<usize>,
}

pub struct ArenaList<T> {
    /// 메모리 풀
    nodes: Vec<Node<T>>,
    /// 삭제 후 재사용 가능한 인덱스 목록
    free_list: Vec<usize>,
    head: Option<usize>,
    len: usize,
}

impl<T> ArenaList<T> {
    pub fn new() -> ArenaList<T> {
        ArenaList {
            nodes: Vec::new(),
            free_list: Vec::new(),
            head: None,
            len: 0,
        }
    }

    /// 새 노드를 위한 슬롯 인덱스를 확보한다.
    fn alloc(&mut self, data: T, next: Option<usize>) -> usize {
        if let Some(idx) = self.free_list.pop() {
            // 재사용 가능한 슬롯이 있는 경우
            self.nodes[idx] = Node { data, next };
            idx
        } else {
            // 재사용 가능한 슬롯이 없는 경우
            let idx = self.nodes.len();
            self.nodes.push(Node { data, next });
            idx
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Default> ArenaList<T> {
    /// 리스트 앞에 데이터를 넣는다.
    pub fn push_front(&mut self, data: T) {
        let idx = self.alloc(data, self.head);
        self.head = Some(idx);
        self.len += 1;
    }

    /// 리스트 맨 앞 요소를 제거하고 데이터를 반환한다.
    pub fn pop_front(&mut self) -> Option<T> {
        let head_idx = self.head?;
        self.head = self.nodes[head_idx].next;
        self.len -= 1;
        self.free_list.push(head_idx);
        Some(std::mem::replace(
            &mut self.nodes[head_idx].data,
            Default::default(),
        ))
    }

    /// 맨 앞 요소를 참조한다.
    pub fn front(&self) -> Option<&T> {
        let head_idx = self.head?;
        Some(&self.nodes[head_idx].data)
    }

    /// 첫번째로 일치하는 항목을 찾아서 제거한다
    pub fn remove(&mut self, target: &T) -> bool
    where
        T: PartialEq,
    {
        let mut prev = Option::<usize>::None;
        let mut curr = self.head;

        while let Some(idx) = curr {
            if self.nodes[idx].data == *target {
                match prev {
                    None => self.head = self.nodes[idx].next,
                    Some(p) => self.nodes[p].next = self.nodes[idx].next,
                }

                self.free_list.push(idx);

                self.nodes[idx].data = Default::default();
                self.len -= 1;
                return true;
            }

            prev = curr;
            curr = self.nodes[idx].next;
        }

        false
    }
}

pub struct ArenaListIter<'a, T> {
    list: &'a ArenaList<T>,
    curr: Option<usize>,
}

impl<T> ArenaList<T> {
    pub fn iter(&self) -> ArenaListIter<'_, T> {
        ArenaListIter {
            list: self,
            curr: self.head,
        }
    }
}

impl<'a, T> Iterator for ArenaListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.map(|idx| {
            self.curr = self.list.nodes[idx].next;
            &self.list.nodes[idx].data
        })
    }
}
