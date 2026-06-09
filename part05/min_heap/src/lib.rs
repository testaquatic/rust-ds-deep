#[derive(Debug)]
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 최솟값 참조
    pub fn peak(&self) -> Option<&T> {
        self.data.first()
    }

    /// 삽입: 끝에 추가
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        let last = self.data.len() - 1;
        self.shift_up(last);
    }

    /// 최솟값 꺼내기: 루트 제거 후 shift_down
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let min = self.data.pop();

        if !self.data.is_empty() {
            self.shift_down(0);
        }

        min
    }

    /// 인덱스 i의 원소를 부모와 비교해서 위로 올린다
    fn shift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;

            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    /// 인덱스 i의 원소를 자식과 비교해서 아래로 내린다
    pub fn shift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            let mut smallest = i;

            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == i {
                break;
            }

            self.data.swap(i, smallest);
            i = smallest;
        }
    }

    /// 기존 Vec에서 힙을 만든다.
    pub fn from_vec(data: Vec<T>) -> Self {
        let mut heap = MinHeap { data };
        if heap.data.iter().len() <= 1 {
            return heap;
        }

        let last_internal = (heap.data.len() - 2) / 2;

        for i in (0..=last_internal).rev() {
            heap.shift_down(i);
        }

        heap
    }
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        MinHeap::new()
    }
}
