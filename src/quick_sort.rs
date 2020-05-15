use rand::{thread_rng, Rng};

pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot = _partition(arr);
        quick_sort(&mut arr[..pivot]);
        quick_sort(&mut arr[pivot + 1..]);
    }
}

pub fn random_quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot = _random_partition(arr);
        random_quick_sort(&mut arr[..pivot]);
        random_quick_sort(&mut arr[pivot + 1..]);
    }
}

fn _random_partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let mut rng = thread_rng();
    let last_index = arr.len() - 1;
    // random a point to be a pivot and swap it to the end
    let pivot = rng.gen_range(0, last_index + 1);
    if last_index != pivot {
        arr.swap(last_index, pivot);
    }
    _partition(arr)
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

#[cfg(test)]
mod test_random {
    use super::random_quick_sort;
    use crate::test_sorting;

    test_sorting!(random_quick_sort);
}
