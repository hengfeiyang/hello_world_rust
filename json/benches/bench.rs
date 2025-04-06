use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use json::{serde, simd};

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("json");
    group.measurement_time(Duration::from_secs(8));
    let json1 = r#"
    {
        "a": 1,
        "b.2": {
            "c": 2,
            "d": {"e": 3,"f": 4,"g": 5,"h": 6,"i": 7,"j": 8,"k": 9,"l": 10,"m": 11}
        }
    }"#;
    let json2 = r#"
    {
        "a": 1,
        "b.2": {
            "c": 2,
            "d": {"e": 3,"f": 4,"g": 5,"h": 6,"i": 7,"j": 8,"k": 9,"l": 10,"m": 11},
            "e": {"e2": {"e3": {"e4": {"e5": {"e6": {"e7": {"e8": {"e9": {"e10": {"e11": "v12"}}}}}}}}}}
        }
    }"#;
    let json1: serde_json::Value = serde_json::from_str(json1).unwrap();
    let json2: serde_json::Value = serde_json::from_str(json2).unwrap();
    for alias in ["serde", "simd"] {
        let h = match alias {
            "serde" => serde::to_vec,
            "simd" => simd::to_vec,
            _ => panic!("not support version"),
        };
        group.bench_function(
            BenchmarkId::from_parameter(format!("{alias}-simple-to_vec")),
            |b| {
                b.iter(|| {
                    let _ = h(black_box(&json1));
                })
            },
        );
        group.bench_function(
            BenchmarkId::from_parameter(format!("{alias}-complex-to_vec")),
            |b| {
                b.iter(|| {
                    let _ = h(black_box(&json2));
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
