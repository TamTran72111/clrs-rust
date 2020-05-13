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
    use super::insertion_sort;
    use crate::test_sorting;

    test_sorting!(insertion_sort);
}
