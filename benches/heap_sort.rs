use clrs_rust::heap_sort::heap_sort;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn bench_1000_shuffled_integers(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut arr: Vec<i32> = (0..1000).collect();
    arr.shuffle(&mut rng);
    c.bench_function("heap sort 1000 shuffled elements", move |b| {
        b.iter_batched(
            || arr.clone(),
            |mut data| heap_sort(black_box(&mut data)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_1000_sorted_integers(c: &mut Criterion) {
    let mut arr: Vec<i32> = (0..1000).collect();
    c.bench_function("heap sort 1000 sorted integers", |b| {
        b.iter(|| heap_sort(black_box(&mut arr)))
    });
}

fn bench_1000_descending_sorted_integers(c: &mut Criterion) {
    let arr: Vec<i32> = (0..1000).rev().collect();
    c.bench_function("heap sort 1000 descending sorted integers", move |b| {
        b.iter_batched(
            || arr.clone(),
            |mut data| heap_sort(black_box(&mut data)),
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
