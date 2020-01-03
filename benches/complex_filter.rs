#[macro_use]
extern crate criterion;
extern crate filters;

use criterion::{criterion_group, criterion_main, Criterion};
use filters::filter::Filter;

fn criterion_benchmark(c: &mut Criterion) {
    let data: Vec<usize> = (0..1_000_000).collect();

    let complex_filter = (|&&a: &&usize|{ a > 5 }).and_not(|&&a: &&usize| a < 20).or(|&&a: &&usize| a == 10);

    c.bench_function("and baseline", |b| b.iter(|| data.iter().filter(|&&a| ((a > 5) && !(a < 20) ) || a == 10).count()));
    c.bench_function("and filter", |b| b.iter(|| data.iter().filter(|a| complex_filter.filter(a)).count()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);