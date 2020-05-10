pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    build_max_heap(arr);
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        max_heapify(&mut arr[..i], 0);
    }
}

pub fn build_max_heap<T: PartialOrd>(arr: &mut [T]) {
    for i in (0..arr.len() / 2).rev() {
        max_heapify(arr, i);
    }
}

pub fn max_heapify<T: PartialOrd>(arr: &mut [T], i: usize) {
    let left = i * 2;
    let right = i * 2 + 1;
    let mut largest = if left < arr.len() && arr[left] > arr[i] {
        left
    } else {
        i
    };
    if right < arr.len() && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(largest, i);
        max_heapify(arr, largest);
    }
}

#[cfg(test)]
mod test {
    use super::heap_sort as sort;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn test_sort_an_empty_array() {
        let mut arr: [i32; 0] = [];
        sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_sort_one_element_array() {
        let mut arr = [1];
        sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn test_sort_an_unsorted_array() {
        let mut arr = [4, 1, 7, 2, 4, 3];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 4, 7]);
    }

    #[test]
    fn test_sort_a_sorted_array() {
        let mut arr = [-4, 2, 3, 4, 4, 7];
        sort(&mut arr);
        assert_eq!(arr, [-4, 2, 3, 4, 4, 7]);
    }

    #[test]
    fn test_sort_a_descending_array() {
        let mut arr = [4, 3, 0, -4, -6, -10];
        sort(&mut arr);
        assert_eq!(arr, [-10, -6, -4, 0, 3, 4]);
    }

    #[test]
    fn test_sort_a_vec() {
        let mut arr = vec![3, 25, 23, 24, 4, 6, 1, 2];
        sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 6, 23, 24, 25]);
    }

    #[test]
    fn test_sort_1000_shuffled_integers() {
        let mut rng = thread_rng();
        let mut arr: Vec<i32> = (0..1_000).collect();
        let sorted = arr.clone();
        arr.shuffle(&mut rng);
        sort(&mut arr);
        assert_eq!(arr, sorted);
    }
}
