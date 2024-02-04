use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use hash::fnv;

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("idgen");
    group.measurement_time(Duration::from_secs(8));
    let key = "hello";
    for alias in ["fnv"] {
        let h = match alias {
            "fnv" => fnv::new_default_hasher,
            _ => panic!("not support version"),
        };
        group.bench_function(BenchmarkId::from_parameter(format!("{alias}-sum64")), |b| {
            let hash = h();
            b.iter(|| hash.sum64(black_box(key)))
        });

        let bucket_num = 32;
        let bucket_mask = h().get_bucket_mask(bucket_num);
        group.bench_function(BenchmarkId::from_parameter(format!("bucket_v1")), |b| {
            b.iter(|| {
                let hash = h();
                hash.bucket_v1(
                    black_box(key),
                    black_box(bucket_num),
                    black_box(bucket_mask),
                )
            })
        });
        group.bench_function(BenchmarkId::from_parameter(format!("bucket_v2")), |b| {
            b.iter(|| {
                let hash = h();
                hash.bucket_v2(
                    black_box(key),
                    black_box(bucket_num),
                    black_box(bucket_mask),
                )
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
