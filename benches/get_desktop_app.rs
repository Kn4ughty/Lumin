use criterion::{criterion_group, criterion_main, Criterion};

use lumin::apps::get_apps;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get apps", |b| b.iter(|| get_apps()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

