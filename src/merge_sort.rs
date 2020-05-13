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
    use super::merge_sort;
    use crate::test_sorting;

    test_sorting!(merge_sort);
}
