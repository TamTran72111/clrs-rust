pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        for j in (0..i).rev() {
            if arr[j + 1] > arr[j] {
                break;
            }
            arr.swap(j, j + 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::insertion_sort as sort;
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
