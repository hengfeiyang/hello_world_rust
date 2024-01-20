use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::criterion::{Output, PProfProfiler};
use promql_parser::parser::token;
use std::time::Duration;

use matchmap::{stdmap, stdmatch};

pub fn ben_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("matchmap");
    group.measurement_time(Duration::from_secs(8));
    let op = promql_parser::parser::token::TokenType::new(token::T_SUM);
    for alias in ["match", "stdmap"] {
        let h = match alias {
            "match" => stdmatch::run,
            "stdmap" => stdmap::run,
            _ => panic!("not support version"),
        };
        group.bench_function(BenchmarkId::from_parameter(format!("{alias}-run")), |b| {
            b.iter(|| {
                let _ = h(black_box(&op));
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
