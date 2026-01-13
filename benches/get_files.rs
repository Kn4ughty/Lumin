use criterion::{Criterion, criterion_group, criterion_main};

use pprof::criterion::{Output, PProfProfiler};
use pprof::flamegraph;

use lumin::files;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("get_files", |b| {
        b.iter(|| {
            //fo
            files::FileSearcher::find_files()
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(Some(flamegraph::Options::default()))));
    targets = criterion_benchmark
}
criterion_main!(benches);
