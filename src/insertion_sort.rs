pub fn insertion_sort<T: PartialOrd>(container: &mut [T]) {
    for i in 1..container.len() {
        for j in (0..i).rev() {
            if container[j + 1] > container[j] {
                break;
            }
            container.swap(j, j + 1);
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
        let mut container: [i32; 0] = [];
        sort(&mut container);
        assert_eq!(container, []);
    }

    #[test]
    fn test_sort_one_element_array() {
        let mut container = [1];
        sort(&mut container);
        assert_eq!(container, [1]);
    }

    #[test]
    fn test_sort_an_unsorted_array() {
        let mut container = [4, 1, 7, 2, 4, 3];
        sort(&mut container);
        assert_eq!(container, [1, 2, 3, 4, 4, 7]);
    }

    #[test]
    fn test_sort_a_sorted_array() {
        let mut container = [-4, 2, 3, 4, 4, 7];
        sort(&mut container);
        assert_eq!(container, [-4, 2, 3, 4, 4, 7]);
    }

    #[test]
    fn test_sort_a_descending_array() {
        let mut container = [4, 3, 0, -4, -6, -10];
        sort(&mut container);
        assert_eq!(container, [-10, -6, -4, 0, 3, 4]);
    }

    #[test]
    fn test_sort_a_vec() {
        let mut container = vec![3, 25, 23, 24, 4, 6, 1, 2];
        sort(container.as_mut_slice());
        assert_eq!(container, vec![1, 2, 3, 4, 6, 23, 24, 25]);
    }

    #[test]
    fn test_sort_1000_shuffled_integers() {
        let mut rng = thread_rng();
        let mut container: Vec<i32> = (0..1_000).collect();
        let sorted = container.clone();
        container.shuffle(&mut rng);
        sort(&mut container);
        assert_eq!(container, sorted);
    }
}
