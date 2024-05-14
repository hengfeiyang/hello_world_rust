use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("mutex");
    group.measurement_time(Duration::from_secs(8));

    let rt = tokio::runtime::Runtime::new().unwrap();

    let fu_mutex = futures::lock::Mutex::new(0);
    group.bench_function(
        BenchmarkId::from_parameter(format!("futures::mutex")),
        |b| {
            b.to_async(&rt).iter(|| async {
                let mut lock = fu_mutex.lock().await;
                *lock += 1;
            });
        },
    );
    let tk_mutex = tokio::sync::Mutex::new(0);
    group.bench_function(BenchmarkId::from_parameter(format!("tokio::mutex")), |b| {
        b.to_async(&rt).iter(|| async {
            let mut lock = tk_mutex.lock().await;
            *lock += 1;
        });
    });
    let tk_rwlock = tokio::sync::RwLock::new(0);
    group.bench_function(
        BenchmarkId::from_parameter(format!("tokio::RwLock:w")),
        |b| {
            b.to_async(&rt).iter(|| async {
                let mut lock = tk_rwlock.write().await;
                *lock += 1;
            });
        },
    );
    let tk_rwlock_r = tokio::sync::RwLock::new(0);
    group.bench_function(
        BenchmarkId::from_parameter(format!("tokio::RwLock:r")),
        |b| {
            b.to_async(&rt).iter(|| async {
                let lock = tk_rwlock_r.read().await;
                _ = *lock;
            });
        },
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = ben_benchmark
}

criterion_main!(benches);
