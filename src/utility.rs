#[macro_export]
macro_rules! test_sorting {
    ($func_name: ident) => {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        #[test]
        fn test_sort_an_empty_array() {
            let mut arr: [i32; 0] = [];
            $func_name(&mut arr);
            assert_eq!(arr, []);
        }

        #[test]
        fn test_sort_one_element_array() {
            let mut arr = [1];
            $func_name(&mut arr);
            assert_eq!(arr, [1]);
        }

        #[test]
        fn test_sort_an_unsorted_array() {
            let mut arr = [4, 1, 7, 2, 4, 3];
            $func_name(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 4, 7]);
        }

        #[test]
        fn test_sort_a_sorted_array() {
            let mut arr = [-4, 2, 3, 4, 4, 7];
            $func_name(&mut arr);
            assert_eq!(arr, [-4, 2, 3, 4, 4, 7]);
        }

        #[test]
        fn test_sort_a_descending_array() {
            let mut arr = [4, 3, 0, -4, -6, -10];
            $func_name(&mut arr);
            assert_eq!(arr, [-10, -6, -4, 0, 3, 4]);
        }

        #[test]
        fn test_sort_a_vec() {
            let mut arr = vec![3, 25, 23, 24, 4, 6, 1, 2];
            $func_name(arr.as_mut_slice());
            assert_eq!(arr, vec![1, 2, 3, 4, 6, 23, 24, 25]);
        }

        #[test]
        fn test_sort_1000_shuffled_integers() {
            let mut rng = thread_rng();
            let mut arr: Vec<i32> = (0..1_000).collect();
            let sorted = arr.clone();
            arr.shuffle(&mut rng);
            $func_name(&mut arr);
            assert_eq!(arr, sorted);
        }
    };
}

#[macro_export]
macro_rules! bench_sorting {
    ($func_name: expr, $func:ident ) => {
        use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
        use rand::rngs::StdRng;
        use rand::seq::SliceRandom;
        use rand::SeedableRng;

        fn bench_1000_shuffled_integers(c: &mut Criterion) {
            let seed = [0; 32];
            let mut rng = StdRng::from_seed(seed);
            let mut arr: Vec<i32> = (0..1000).collect();
            arr.shuffle(&mut rng);
            let name = format!("{} sort 1000 shuffled elements", $func_name);
            c.bench_function(name.as_str(), move |b| {
                b.iter_batched(
                    || arr.clone(),
                    |mut data| $func(black_box(&mut data)),
                    BatchSize::SmallInput,
                )
            });
        }

        fn bench_1000_sorted_integers(c: &mut Criterion) {
            let mut arr: Vec<i32> = (0..1000).collect();
            let name = format!("{} sort 1000 sorted integers", $func_name);
            c.bench_function(name.as_str(), move |b| {
                b.iter(|| $func(black_box(&mut arr)))
            });
        }

        fn bench_1000_descending_sorted_integers(c: &mut Criterion) {
            let arr: Vec<i32> = (0..1000).rev().collect();
            let name = format!("{} sort 1000 descending sorted integers", $func_name);
            c.bench_function(name.as_str(), move |b| {
                b.iter_batched(
                    || arr.clone(),
                    |mut data| $func(black_box(&mut data)),
                    BatchSize::SmallInput,
                )
            });
        }

        criterion_group!(
            benches,
            bench_1000_shuffled_integers,
            bench_1000_sorted_integers,
            bench_1000_descending_sorted_integers
        );
        criterion_main!(benches);
    };
}
