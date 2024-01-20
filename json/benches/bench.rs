use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use json::{v1, v2};

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("json");
    group.measurement_time(Duration::from_secs(8));
    let json = r#"
    {
        "a": 1,
        "b.2": {
            "c": 2,
            "d": {
                "e": 3
            }
        }
    }"#;
    let json: serde_json::Value = serde_json::from_str(json).unwrap();
    for alias in ["v1", "v2"] {
        let h = match alias {
            "v1" => v1::to_vec,
            "v2" => v2::to_vec,
            _ => panic!("not support version"),
        };
        group.bench_function(
            BenchmarkId::from_parameter(format!("{alias}-to_vec")),
            |b| {
                b.iter(|| {
                    let _ = h(black_box(&json));
                })
            },
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = ben_benchmark
}

criterion_main!(benches);
