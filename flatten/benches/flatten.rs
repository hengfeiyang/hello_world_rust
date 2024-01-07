use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use flatten::{v1, v2, v3};

pub fn flatten_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("flatten");
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
    let json = r#"
    {
        "a": 1,
        "b_2": 3,
        "x": "bb"
    }"#;
    let json: serde_json::Value = serde_json::from_str(json).unwrap();
    for alias in ["flatten_lib", "v0.7.2", "v_next"] {
        let h = match alias {
            "flatten_lib" => v1::flatten,
            "v0.7.2" => v2::flatten,
            "v_next" => v3::flatten,
            _ => panic!("not support version"),
        };
        group.bench_function(
            BenchmarkId::from_parameter(format!("{alias}-flatten")),
            |b| {
                b.iter(|| {
                    let _ = h(black_box(json.clone()));
                })
            },
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = flatten_benchmark
}

criterion_main!(benches);
