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
    use super::heap_sort;
    use crate::test_sorting;

    test_sorting!(heap_sort);
}
