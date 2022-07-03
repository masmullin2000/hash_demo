#![allow(unused_imports)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use lib::*;

fn run_test(name: &str, c: &mut Criterion) {
    let mut g = c.benchmark_group(name);
    g.bench_function("bench", |b| b.iter(|| black_box(doit()))); 
    g.finish();
}

fn bench(c: &mut Criterion) {
    run_test("first", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
