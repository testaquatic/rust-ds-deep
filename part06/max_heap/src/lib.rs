pub struct MaxHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.shift_up(self.data.len() - 1);
    }

    fn shift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] > self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    pub fn peak(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let max_val = self.data.pop();

        if !self.data.is_empty() {
            self.shift_down(0);
        }

        max_val
    }

    fn shift_down(&mut self, mut idx: usize) {
        let len = self.data.len();

        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < len && self.data[right] > self.data[largest] {
                largest = right;
            }

            if largest == idx {
                break;
            }

            self.data.swap(idx, largest);
            idx = largest;
        }
    }
}

pub fn heap_sort<T: Ord>(data: &mut Vec<T>) {
    let n = data.len();
    for i in (0..n / 2).rev() {
        shift_down_sort(data, i, n);
    }

    for end in (1..n).rev() {
        data.swap(0, end);
        shift_down_sort(data, 0, end);
    }
}

fn shift_down_sort<T: Ord>(data: &mut Vec<T>, mut idx: usize, heap_size: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest = idx;

        if left < heap_size && data[left] > data[largest] {
            largest = left;
        }
        if right < heap_size && data[right] > data[largest] {
            largest = right;
        }

        if largest == idx {
            break;
        }

        data.swap(idx, largest);
        idx = largest;
    }
}
