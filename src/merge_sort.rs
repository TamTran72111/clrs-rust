pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let middle = (arr.len() - 1) / 2;
        merge_sort(&mut arr[..=middle]);
        merge_sort(&mut arr[middle + 1..]);
        _merge(arr);
    }
}

fn _merge<T: PartialOrd + Copy>(arr: &mut [T]) {
    let middle = (arr.len() - 1) / 2;
    let left: Vec<T> = (0..=middle).map(|i| arr[i]).collect();
    let right: Vec<T> = (middle + 1..arr.len()).map(|i| arr[i]).collect();

    let mut k = 0;
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod test {
    use super::merge_sort as sort;
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
