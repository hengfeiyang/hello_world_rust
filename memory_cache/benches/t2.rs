use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use memory_cache::memory_v1 as memory;

pub fn norm_sine_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("memory");
    group.measurement_time(Duration::from_secs(8));
    for alias in ["v1", "v2", "v3"] {
        let h = match alias {
            "v1" => memory::download,
            "v2" => memory::download,
            "v3" => memory::download,
            _ => panic!("not support version"),
        };
        group.bench_function(
            BenchmarkId::from_parameter(format!("{alias}-download")),
            |b| {
                b.to_async(&rt).iter(|| async {
                    let session_id = "test";
                    let file = "test.parquet";
                    let _ = h(black_box(session_id), black_box(file)).await;
                })
            },
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = norm_sine_benchmark
}

criterion_main!(benches);
