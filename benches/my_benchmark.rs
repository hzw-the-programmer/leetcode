use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use leetcode::lc3120::number_of_special_chars;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("number_of_special_chars aaAbcBC", |b| b.iter(|| number_of_special_chars(black_box("aaAbcBC".into()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);