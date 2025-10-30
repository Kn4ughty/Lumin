use criterion::{Criterion, criterion_group, criterion_main};

use pprof::criterion::{Output, PProfProfiler};
use pprof::flamegraph;

use lumin::apps::desktop_entry::get_apps;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get apps", |b| b.iter(|| get_apps()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(Some(flamegraph::Options::default()))));
    targets = criterion_benchmark
}
criterion_main!(benches);
