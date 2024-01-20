use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use idgen::{v1, v2, v3};

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("idgen");
    group.measurement_time(Duration::from_secs(8));
    for alias in ["v1", "v2", "v3"] {
        let h = match alias {
            "v1" => v1::generate,
            "v2" => v2::generate,
            "v3" => v3::generate,
            _ => panic!("not support version"),
        };
        group.bench_function(BenchmarkId::from_parameter(format!("{alias}-gen")), |b| {
            b.iter(|| {
                let _ = h();
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = ben_benchmark
}

criterion_main!(benches);
