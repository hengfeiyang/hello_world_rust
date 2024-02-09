use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use hash::*;
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("idgen");
    group.measurement_time(Duration::from_secs(8));
    let key = "hello";
    for alias in ["fnv", "defaultHash", "xxhash", "murmur3", "cityhash","gxhash"] {
        let mut h: Box<dyn Sum64> = match alias {
            "fnv" => Box::new(fnv::new()),
            "defaultHash" => Box::new(default_hasher::new()),
            "xxhash" => Box::new(xxhash::new()),
            "murmur3" => Box::new(murmur3::new()),
            "cityhash" => Box::new(cityhash::new()),
            "gxhash" => Box::new(gxhash::new()),
            _ => panic!("not support version"),
        };
        group.bench_function(BenchmarkId::from_parameter(format!("{alias}-sum64")), |b| {
            b.iter(|| h.sum64(black_box(key)))
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = ben_benchmark
}

criterion_main!(benches);
