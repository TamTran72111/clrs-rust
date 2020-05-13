pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot = _partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }
}

fn _partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let last = arr.len() - 1;
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[last] {
            if i != j {
                arr.swap(i, j);
            }
            i += 1;
        }
    }
    if i != last {
        arr.swap(i, last);
    }
    return i;
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    use crate::test_sorting;

    test_sorting!(quick_sort);
}
