pub fn find_maximum_subarray(arr: &[i32], low: usize, high: usize) -> (i32, usize, usize) {
    if high == low {
        (arr[low], low, high)
    } else {
        let mid = low + (high - low) / 2;
        let (left_sum, left_low, left_high) = find_maximum_subarray(arr, low, mid);
        let (right_sum, right_low, right_high) = find_maximum_subarray(arr, mid + 1, high);
        let (cross_sum, cross_low, cross_high) = _find_max_crossing_subarray(arr, low, mid, high);
        if left_sum >= right_sum && left_sum >= cross_sum {
            (left_sum, left_low, left_high)
        } else if right_sum >= left_sum && right_sum >= cross_sum {
            (right_sum, right_low, right_high)
        } else {
            (cross_sum, cross_low, cross_high)
        }
    }
}

fn _find_max_crossing_subarray(
    arr: &[i32],
    low: usize,
    mid: usize,
    high: usize,
) -> (i32, usize, usize) {
    let mut left_sum = i32::min_value();
    let mut sum = 0;
    let mut max_left = mid;
    for i in (low..mid + 1).rev() {
        sum += arr[i];
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }
    let mut right_sum = i32::min_value();
    sum = 0;
    let mut max_right = mid;
    for i in mid + 1..high + 1 {
        sum += arr[i];
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }
    (left_sum + right_sum, max_left, max_right)
}

#[cfg(test)]
mod test {
    use super::find_maximum_subarray;

    #[test]
    fn test_find_maximum_subarray_of_an_array_of_1_element() {
        let arr = [1];
        assert_eq!(find_maximum_subarray(&arr, 0, 0), (1, 0, 0));
    }

    #[test]
    fn test_find_maximum_subarray_of_an_array_of_nonnegative_elements() {
        let arr = [1, 0, 6, 2, 4, 6, 7, 23];
        assert_eq!(find_maximum_subarray(&arr, 0, 7), (49, 0, 7));
    }

    #[test]
    fn test_find_maximum_subarray_of_an_array_of_nonpositive_elements() {
        let arr = [-1, 0, -6, -2, -4, -6, -7, -23];
        assert_eq!(find_maximum_subarray(&arr, 0, 7), (0, 1, 1));
    }

    #[test]
    fn test_find_maximum_subarray_of_an_array_of_negative_elements() {
        let arr = [-12, -100, -6, -2, -4, -6, -7, -23];
        assert_eq!(find_maximum_subarray(&arr, 0, 7), (-2, 3, 3));
    }

    #[test]
    fn test_find_maximum_subarray_of_an_integer_array() {
        let arr = [
            13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
        ];
        assert_eq!(find_maximum_subarray(&arr, 0, 14), (43, 7, 10));
    }
}
