#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate advent;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("both parts", |b| b.iter(|| advent::advent()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);