/// Arena에서 관리하는 단일 노드
/// [Self::next]는 다음 노드의 인덱스, [None]이면 리스트 끝
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<usize>,
}

/// [Vec]을 메모리 풀로 사용하는 단방향 연결 리스트
/// [Self::free_list]는 삭제 후 재사용 가능한 슬롯 인덱스 목록
#[derive(Debug)]
pub struct ArenaList<T> {
    nodes: Vec<Node<T>>,
    free_list: Vec<usize>,
    head: Option<usize>,
    len: usize,
}

impl<T> ArenaList<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            head: None,
            len: 0,
        }
    }

    /// 새 노드를 위한 슬롯 인덱스를 확보한다.
    /// 재사용 가능한 슬롯이 있으면 그걸 쓰고, 없으면 push
    fn alloc(&mut self, data: T, next: Option<usize>) -> usize {
        if let Some(idx) = self.free_list.pop() {
            self.nodes[idx] = Node { data, next };

            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node { data, next });

            idx
        }
    }

    /// 슬롯을 반환한다. 데이터를 꺼내고 [Self::free_list]에 인덱스를 추가
    fn dealloc(&mut self, idx: usize) -> T {
        // next를 None으로 초기해 dangling 참조 방지
        self.nodes[idx].next = None;
        self.free_list.push(idx);
        // 데이터를 이동시키기 위해 임시 더미값 필요 -> std::mem::replace 활용
        // 하지만 T: Default 제약을 피하고 싶으므로 unsafe 없이 꺼내는 트릭 사용
        // 여기에서 swap으로 구현하지 않고 직접 읽는 방식을 씀
        // 실제로는 T를 MaybeUninit로 감싸는 게 정석이지만
        // 입문 예제이브로 Default 바운드를 허용함
        todo!("dealloc은 push_front/remove 구현에서 통합 처리");
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Default> ArenaList<T> {
    /// 리스트 앞에 데이터를 삽입한다.
    pub fn push_front(&mut self, data: T) {
        let idx = self.alloc(data, self.head);
        self.head = Some(idx);
        self.len += 1;
    }

    /// 리스트의 맨 앞 요소를 제거하고 데이터를 반환한다.
    pub fn pop_front(&mut self) -> Option<T> {
        let head_idx = self.head?;
        self.head = self.nodes[head_idx].next;
        self.len -= 1;
        self.free_list.push(head_idx);
        // std::mem::replace로 데이터를 꺼내고 빈 솔롯엔 Default 값을 남긴다.
        let data = std::mem::take(&mut self.nodes[head_idx].data);

        Some(data)
    }

    /// 맨 앞 요소를 참조한다.
    pub fn peak_front(&self) -> Option<&T> {
        let head_idx = self.head?;
        Some(&self.nodes[head_idx].data)
    }

    /// 특정 값을 찾아 제거한다(첫 번째 일치 항목).
    /// [PartialEq]바운드 추가
    pub fn remove(&mut self, target: &T) -> bool
    where
        T: PartialEq,
    {
        let mut prev = Option::<usize>::None;
        let mut curr = self.head;

        while let Some(idx) = curr {
            if self.nodes[idx].data == *target {
                // 이전 노드 또는 head 갱신
                match prev {
                    None => self.head = self.nodes[idx].next,
                    Some(p) => self.nodes[p].next = self.nodes[idx].next,
                }
                self.free_list.push(idx);
                // 데이터 자리를 Default로 채워 메모리 정리
                self.nodes[idx].data = T::default();
                self.nodes[idx].next = None;
                self.len -= 1;

                return true;
            }

            prev = curr;
            curr = self.nodes[idx].next;
        }

        false
    }
}

impl<T> Default for ArenaList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 불변 순회를 위한 이터레이터
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
        let idx = self.curr?;
        self.curr = self.list.nodes[idx].next;

        Some(&self.list.nodes[idx].data)
    }
}
