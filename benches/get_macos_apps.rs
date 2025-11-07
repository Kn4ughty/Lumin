use criterion::{Criterion, criterion_group, criterion_main};

use pprof::criterion::{Output, PProfProfiler};
use pprof::flamegraph;

use lumin::apps::{OSAppSearcher, mac_apps::MacOsAppSearcher};

pub fn criterion_benchmark(c: &mut Criterion) {
    let searcher = MacOsAppSearcher::default();
    c.bench_function("get apps", |b| b.iter(|| searcher.get_apps()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(Some(flamegraph::Options::default()))));
    targets = criterion_benchmark
}
criterion_main!(benches);
