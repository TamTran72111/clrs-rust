use clrs_rust::insertion_sort::insertion_sort;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn bench_1000_shuffled_integers(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut container: Vec<i32> = (0..1000).collect();
    container.shuffle(&mut rng);
    c.bench_function("insertion sort 1000 shuffled elements", move |b| {
        b.iter_batched(
            || container.clone(),
            |mut data| insertion_sort(black_box(&mut data)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_1000_sorted_integers(c: &mut Criterion) {
    let mut container: Vec<i32> = (0..1000).collect();
    c.bench_function("insertion sort 1000 sorted integers", |b| {
        b.iter(|| insertion_sort(black_box(&mut container)))
    });
}

fn bench_1000_descending_sorted_integers(c: &mut Criterion) {
    let container: Vec<i32> = (0..1000).rev().collect();
    c.bench_function(
        "insertion sort 1000 descending sorted integers",
        move |b| {
            b.iter_batched(
                || container.clone(),
                |mut data| insertion_sort(black_box(&mut data)),
                BatchSize::SmallInput,
            )
        },
    );
}

criterion_group!(
    benches,
    bench_1000_shuffled_integers,
    bench_1000_sorted_integers,
    bench_1000_descending_sorted_integers
);
criterion_main!(benches);
