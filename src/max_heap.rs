use crate::heap_sort::max_heapify;

struct MaxHeap<T> {
    arr: Vec<T>,
}

impl<T: PartialOrd> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap { arr: vec![] }
    }

    pub fn max(&self) -> Option<&T> {
        if self.arr.is_empty() {
            None
        } else {
            Some(&self.arr[0])
        }
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.arr.is_empty() {
            None
        } else {
            let last_index = self.arr.len() - 1;
            self.arr.swap(0, last_index);
            let max = self.arr.pop();
            max_heapify(&mut self.arr, 0);
            max
        }
    }

    pub fn increase_key(&mut self, index: usize, key: T) {
        // move the element of the current index up the tree until it is in correct position
        if index < self.arr.len() && self.arr[index] < key {
            self.arr[index] = key;
            self.level_up(index);
        }
    }

    fn level_up(&mut self, mut index: usize) {
        let mut parent_index = index / 2;
        while index > 0 && self.arr[parent_index] < self.arr[index] {
            self.arr.swap(index, parent_index);
            index = parent_index;
            parent_index /= 2;
        }
    }

    fn insert(&mut self, key: T) {
        self.arr.push(key);
        self.level_up(self.arr.len() - 1);
    }
}

#[cfg(test)]
mod test {
    use super::MaxHeap;

    #[test]
    fn test_max_heap_of_empty() {
        let mut max_heap: MaxHeap<i32> = MaxHeap::new();
        assert_eq!(max_heap.max(), None);
        assert_eq!(max_heap.extract_max(), None);
    }

    #[test]
    fn test_max_heap_of_1_element() {
        let mut max_heap = MaxHeap::new();
        assert_eq!(max_heap.max(), None);
        max_heap.insert(5);
        assert_eq!(max_heap.max(), Some(&5));
        assert_eq!(max_heap.extract_max(), Some(5));
        assert_eq!(max_heap.extract_max(), None);
        assert_eq!(max_heap.extract_max(), None);
    }

    #[test]
    fn test_max_heap_of_3_elements() {
        let mut max_heap = MaxHeap::new();
        assert_eq!(max_heap.max(), None);
        max_heap.insert(5);
        assert_eq!(max_heap.max(), Some(&5));
        max_heap.insert(3);
        assert_eq!(max_heap.max(), Some(&5));
        max_heap.insert(50);
        assert_eq!(max_heap.max(), Some(&50));
        assert_eq!(max_heap.extract_max(), Some(50));
        assert_eq!(max_heap.extract_max(), Some(5));
        assert_eq!(max_heap.max(), Some(&3));
        assert_eq!(max_heap.extract_max(), Some(3));
        assert_eq!(max_heap.max(), None);
        assert_eq!(max_heap.extract_max(), None);
    }
}
